
extern mod glfw;
extern mod opengles;
extern mod extra;

use std::io;
use std::os;
use std::comm;
use std::task;
use std::uint;
use extra::json;
use extra::net;
use extra::uv_global_loop;

use math::Vec3;
use gl::camera::Camera;
use gl::obj::Mesh;

use es = opengles::gl2;

#[path = "math/mod.rs"]
mod math;

#[path = "gl/mod.rs"]
mod gl;

#[macro_escape]
mod check;

static SCREEN_WIDTH: uint = 800;
static SCREEN_HEIGHT: uint = 600;

fn main() {
    #[main];
    
    do glfw::spawn {
        let rods_file = match os::args() {
            [_, path] => path,
            _ => ~"default-rods.json"
        };

        let rods_reader = match io::file_reader(&Path(rods_file)) {
            Ok(reader) => reader,
            Err(error) => fail!(error)
        };
        
        let rods_specs = match json::from_reader(rods_reader) {
            Ok(json) => match json {
                json::List(list) => list,
                _ => fail!("Invalid JSON")
            },
            Err(_) => fail!("Invalid JSON")
        };
        
        let ports = start_server(rods_specs.len() as u8);
        
        glfw::window_hint::context_version(3, 2);
        glfw::window_hint::opengl_profile(glfw::OPENGL_CORE_PROFILE);
        glfw::window_hint::opengl_forward_compat(true);
        glfw::window_hint::samples(8);
        
        let window = ~glfw::Window::create(SCREEN_WIDTH, SCREEN_HEIGHT, "Light Sculpture Simulator", glfw::Windowed).unwrap();
        
        window.make_context_current();
        
        let vao = check!(es::gen_vertex_arrays(1)[0]);
        check!(es::bind_vertex_array(vao));

        let mut camera = Camera::new(window, "shaders/everything");
        let base = Mesh::gen_base();
        camera.add_mesh(base);
        
        for rods_specs.consume_iter().zip(ports.consume_iter()).advance |(json, port)| {
            match json {
                json::Object(obj) => {
                    let (x, y) = if obj.contains_key_equiv(&("x")) && obj.contains_key_equiv(&("y")) {
                        (match obj.find_equiv(&("x")) {
                            Some(val) => match *val {
                                json::Number(num) => num,
                                _ => fail!("Invalid JSON")
                            },
                            None => fail!("Invalid JSON")
                        }, match obj.find_equiv(&("y")) {
                            Some(val) => match *val {
                                json::Number(num) => num,
                                _ => fail!("Invalid JSON")
                            },
                            None => fail!("Invalid JSON")
                        })
                    } else if obj.contains_key_equiv(&("angle")) && obj.contains_key_equiv(&("radius")) {
                        let angle = match obj.find_equiv(&("angle")) {
                            Some(val) => match *val {
                                json::Number(num) => num,
                                _ => fail!("Invalid JSON")
                            },
                            None => fail!("Invalid JSON")
                        };
                        let radius = match obj.find_equiv(&("radius")) {
                            Some(val) => match *val {
                                json::Number(num) => num,
                                _ => fail!("Invalid JSON")
                            },
                            None => fail!("Invalid JSON")
                        };
                        (radius * angle.cos(), radius * angle.sin())
                    } else {
                        fail!("Invalid JSON")
                    };
                    let height = match obj.find_equiv(&("height")) {
                        Some(val) => match *val {
                            json::Number(num) => num,
                            _ => fail!("Invalid JSON")
                        },
                        None => fail!("Invalid JSON")
                    };
                    camera.add_mesh(Mesh::better_rod((x / 10.0) as f32,
                                                     (y / 10.0) as f32,
                                                     (height / 10.0) as f32,
                                                     port))
                },
                _ => fail!("Invalid JSON")
            }
        }
        
        camera.look_at(Vec3::new(0.0f32, 1.0, 0.0), Vec3::new(0.0, -2.0, -2.0), Vec3::new(0.0, 0.0, 1.0));
        camera.perspective(3.14159 / 4.0f32, 0.1, 10.0);
        
        es::enable(es::BLEND);
        es::enable(es::DEPTH_TEST);
        es::blend_func(es::SRC_ALPHA, es::ONE_MINUS_SRC_ALPHA);

        while !camera.should_close() {
            glfw::poll_events();
            
            if camera.is_key_down(glfw::KEY_Q) {
                camera.translate(Vec3::new(0.0, 0.05, 0.0));
            }
            if camera.is_key_down(glfw::KEY_E) {
                camera.translate(Vec3::new(0.0, -0.05, 0.0));
            }
            if camera.is_key_down(glfw::KEY_D) {
                camera.translate(Vec3::new(0.05, 0.0, 0.0));
            }
            if camera.is_key_down(glfw::KEY_A) {
                camera.translate(Vec3::new(-0.05, 0.0, 0.0));
            }
            if camera.is_key_down(glfw::KEY_W) {
                camera.translate(Vec3::new(0.0, 0.0, -0.05));
            }
            if camera.is_key_down(glfw::KEY_S) {
                camera.translate(Vec3::new(0.0, 0.0, 0.05));
            }
            if camera.is_key_down(glfw::KEY_DOWN) {
                camera.rotate(3.14159 / 40.0, 0.0, 0.0);
            }
            if camera.is_key_down(glfw::KEY_UP) {
                camera.rotate(3.14159 / -40.0, 0.0, 0.0);
            }
            if camera.is_key_down(glfw::KEY_LEFT) {
                camera.rotate(0.0, 3.14159 / 40.0, 0.0);
            }
            if camera.is_key_down(glfw::KEY_RIGHT) {
                camera.rotate(0.0, 3.14159 / -40.0, 0.0);
            }
            
            es::clear_color(0.9, 0.9, 0.9, 1.0);
            es::clear(es::COLOR_BUFFER_BIT | es::DEPTH_BUFFER_BIT);
            
            camera.draw();
        }
    }
}

fn start_server(num_leds: u8) -> ~[comm::Port<(f32, f32, f32)>] {
    let mut ports: ~[comm::Port<(f32, f32, f32)>] = ~[];
    let mut chans: ~[comm::SharedChan<(f32, f32, f32)>] = ~[];
    for (num_leds as uint).times {
        let (port, chan) = comm::stream();
        ports.push(port);
        chans.push(comm::SharedChan::new(chan));
    }
    let chans = chans;
    
    let iotask = uv_global_loop::get();
    let localhost = net::ip::v4::parse_addr("127.0.0.1");
    
    do task::spawn {
        
        println("um");
        let cloned_chans = chans.clone();
        do net::tcp::listen(localhost, 7654, 100, &iotask, |_| ()) |new_conn, _| {
            let my_chans = cloned_chans.clone();
            do task::spawn {
                let accept_result = net::tcp::accept(new_conn);
                let sock = match accept_result {
                    Ok(val) => val,
                    Err(_) => {
                        fail!("Socket error")
                    }
                };
                printfln!("Connection from %s", net::ip::format_addr(&sock.get_peer_addr()));
                let sock_buf = net::tcp::socket_buf(sock);
                loop {
                    let leds = sock_buf.read_byte();
                    if leds < 0 {
                        break;
                    }
                    for uint::range(0, leds as uint) |_| {
                        let bytes = sock_buf.read_bytes(4);
                        let led = bytes[0];
                        if led < num_leds {
                            let r: f32 = (bytes[1] as f32) / 255.0f32;
                            let g: f32 = (bytes[2] as f32) / 255.0f32;
                            let b: f32 = (bytes[3] as f32) / 255.0f32;
                            my_chans[led].send((r, g, b));
                        }
                    }
                }
            }
        };
    }
    return ports;
}