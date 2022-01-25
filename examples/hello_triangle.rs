extern crate cobin;

use {
  std::mem,
  cobin::{
    AutoReleaseContext,
    Strong,
    util,
    runtime,
    runtime::NSObjectBase,
    app_kit::*,
    foundation::*,
    core_graphics,
    metal::*,
    core_animation
  }
};

fn main() {
  let width = 1280;
  let height = 720;

  let app = unsafe {
    let app = NSApplication::shared_application().as_ref().unwrap();
    app.set_activation_policy(NSApplicationActivationPolicy::Regular);
    setup_top_menu(app);
    app.finish_launching();

    #[cfg(debug_assertions)]
    app.activate_ignoring_other_apps(true);

    app
  };

  let window = unsafe { create_window(width, height) };

  let device = unsafe { util::mtl_create_system_default_device().as_mut().unwrap() };

  let layer = unsafe {
    let layer = core_animation::CAMetalLayer::new();
    layer.set_device(&mut *device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_framebuffer_only(false);
    layer
  };

  unsafe {
    let view = window.content_view().as_ref().unwrap();
    view.set_wants_layer(true);
    view.set_layer(layer.as_mut_ptr() as *mut core_animation::CALayer);
  }

  let mut pipeline_state = unsafe {
    let shader_source = include_str!("hello_triangle.metal");
    let shader_library = compile_shader_lib(device, shader_source);
    create_pipeline_state(device, &shader_library)
  };

  let command_queue = unsafe { device.new_command_queue() };

  let mut position_buffer = unsafe {
    let positions: [(f32, f32); 3] = [
      (0.0, 0.75),
      (0.75, -0.75),
      (-0.75, -0.75),
    ];

    device.new_buffer_with_bytes_length_options(
      positions.as_ptr() as *const _,
      mem::size_of_val(&positions),
      MTLResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED | MTLResourceOptions::STORAGE_MODE_MANAGED
    )
  };

  let mut termination_requested = false;
  let mut frame_count = 0;

  while !termination_requested {
    unsafe {
      let _context = AutoReleaseContext::new();

      poll_events(&app, &mut termination_requested);

      let cmd_buffer = command_queue.command_buffer().as_mut().unwrap();

      let drawable = layer.next_drawable();
      let output_texture = (&*drawable).texture().as_mut().unwrap();

      let mut render_pass_descriptor = create_render_pass_descriptor(output_texture);

      let encoder = cmd_buffer.render_command_encoder_with_descriptor(&mut *render_pass_descriptor).as_ref().unwrap();
      encoder.set_render_pipeline_state(&mut *pipeline_state);

      encoder.set_vertex_buffer_offset_at_index(&mut *position_buffer, 0, 0);

      let displacement: (f32, f32) = {
        let radius = 0.1;
        let speed = 0.05;

        (
          radius * f32::cos(frame_count as f32 * speed),
          radius * f32::sin(frame_count as f32 * speed)
        )
      };
      encoder.set_vertex_bytes_offset_at_index(
        &displacement as *const (f32, f32) as *const _,
        mem::size_of_val(&displacement),
        1
      );

      encoder.draw_primitives_vertex_start_vertex_count(
        MTLPrimitiveType::Triangle,
        0,
        3
      );

      encoder.end_encoding();

      frame_count += 1;

      cmd_buffer.present_drawable(drawable as *mut MTLDrawable);
      cmd_buffer.commit();
    }
  }

  println!("Terminated gracefully.");
}

unsafe fn setup_top_menu(app: &NSApplication) {
  let mut app_menu = NSMenu::new();
  {
    let mut title = util::string("Quit");
    let action = cobin::util::selector("terminate:");
    let mut key = util::string("q");
    app_menu.add_item_with_title_action_key_equivalent(&mut *title, action, &mut *key);
  }

  let mut app_item = NSMenuItem::new();
  app_item.set_submenu(&mut *app_menu);

  let mut bar = NSMenu::new();
  bar.add_item(&mut *app_item);

  app.set_main_menu(&mut *bar);
}

unsafe fn create_window(width: usize, height: usize) -> Strong<NSWindow> {
  let content_rect = NSRect {
    origin: core_graphics::CGPoint { x: 0.0, y: 0.0 },
    size: core_graphics::CGSize { width: width as f64, height: height as f64 }
  };
  let mut title = util::string("Hello Triangle");

  let window = NSWindow::new_with_content_rect_style_mask_backing_defer(
    content_rect,
    NSWindowStyleMask::Titled,
    NSBackingStoreType::Buffered,
    false
  );
  window.set_accepts_mouse_moved_events(true);
  window.center();
  window.set_title(&mut *title);
  window.make_key_and_order_front(runtime::NIL);
  window
}

unsafe fn create_pipeline_state(device: &MTLDevice, shader_lib: &MTLLibrary) -> Strong<MTLRenderPipelineState> {

  let mut vertex_shader = shader_lib.new_function_with_name(&mut *util::string("vs"));
  let mut frag_shader = shader_lib.new_function_with_name(&mut *util::string("ps"));

  let mut descriptor = MTLRenderPipelineDescriptor::new();
  descriptor.set_vertex_function(&mut *vertex_shader);
  descriptor.set_fragment_function(&mut *frag_shader);

  let color_attachments = descriptor.color_attachments().as_ref().unwrap();

  let color_attachment_descriptor = color_attachments.object_at_indexed_subscript(0).as_ref().unwrap();
  color_attachment_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);

  let pipeline_state = device.new_render_pipeline_state_with_descriptor_error(&mut *descriptor, 0 as *mut NSError);
  // TODO: Check error.

  assert!(!pipeline_state.is_null());

  pipeline_state
}

pub unsafe fn compile_shader_lib(device: &MTLDevice, source: &str) -> Strong<MTLLibrary> {
  let mut shader_source = util::string(source);
  let mut options = MTLCompileOptions::new();
  let lib = device.new_library_with_source_options_error(&mut *shader_source, &mut *options, 0 as *mut NSError);
  // TODO: Check error.

  assert!(!lib.is_null());

  lib
}

unsafe fn create_render_pass_descriptor(texture: &mut MTLTexture) -> Strong<MTLRenderPassDescriptor> {
  let descriptor = MTLRenderPassDescriptor::new();
  let color_attachments = descriptor.color_attachments().as_ref().unwrap();

  {
    let attachment = color_attachments.object_at_indexed_subscript(0).as_ref().unwrap();
    attachment.set_texture(&mut *texture);
    attachment.set_load_action(MTLLoadAction::Clear);
    attachment.set_store_action(MTLStoreAction::Store);
  }

  descriptor
}

unsafe fn poll_events(app: &NSApplication, termination_requested: &mut bool) {
  loop {
    let event = app.next_event_matching_mask_until_date_in_mode_dequeue(
      NSEventMask::Any,
      NSDate::distant_past(),
      util::ns_default_run_loop_mode(),
      true
    );

    match event.as_mut() {
      None => break,
      Some(event) => {
        match event.event_type() {
          NSEventType::KeyDown => {
            *termination_requested = true
          },
          _ => app.send_event(event)
        }
      }
    }
  }
}
