extern crate diecast;
extern crate liquid;
extern crate typemap;

use std::default::Default;
use std::path::PathBuf;

use diecast::{Handle, Item};
use liquid::{
    LiquidOptions,
    Renderable,
    Context,
};

pub struct RenderTemplate<H>
where H: Fn(&Item) -> Context + Sync + Send + 'static {
    binding: String,
    path: PathBuf,
    handler: H,
}

impl<H> Handle<Item> for RenderTemplate<H>
where H: Fn(&Item) -> Context + Sync + Send + 'static {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        item.body = {
            let mut context = (self.handler)(item);

            println!("looking for path {:?}", self.path);

            let tmp = {
                item.bind().dependencies[&self.binding].items()
                .iter()
                .find(|i|
                    i.route().reading()
                    .map(|p| {
                        println!("checking path {:?}", p);
                        p == self.path.as_ref()
                    })
                    .unwrap_or(false))
            };

            if tmp.is_none() {
                println!("LIQUID: template not found!");
            }

            if let Some(tmp) = tmp {
                let mut options: LiquidOptions = Default::default();
                let template = liquid::parse(&tmp.body, &mut options).unwrap();

                template.render(&mut context).unwrap()
            } else {
                // TODO handle
                String::new()
            }
        };

        Ok(())
    }
}

#[inline]
pub fn render_template<H, D, P>(binding: D, path: P, handler: H) -> RenderTemplate<H>
where H: Fn(&Item) -> Context + Sync + Send + 'static, D: Into<String>, P: Into<PathBuf> {
    RenderTemplate {
        binding: binding.into(),
        path: path.into(),
        handler: handler,
    }
}

// let liquid =
//     Rule::read("liquid")
//     .source(source::select("liquid/*.html".parse::<Glob>().unwrap()))
//     .handler(bind::parallel_each(item::read));

// let liquid_index =
//     Rule::create("liquid index")
//     .depends_on(&posts)
//     .depends_on(&liquid)
//     .source(source::create("liquid.html"))
//     .handler(bind::each(Chain::new()
//         .link(diecast_liquid::render_template(&liquid, "liquid/index.html", |item: &Item| -> Context {
//             let mut context = liquid::Context::new();
//             context.set_val("testing", Value::Str(String::from("it works")));
//             context.set_val("num", Value::Num(5f32));
//             context.set_val("numTwo", Value::Num(6f32));
//             context
//         }))
//         .link(route::pretty)
//         .link(item::write)));

