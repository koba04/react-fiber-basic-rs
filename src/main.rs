struct Fiber<'a> {
    type_: &'a str,
    child: OptionnalFiber<'a>,
}

type OptionnalFiber<'a> = Option<&'a Fiber<'a>>;

fn create_fiber<'a>(type_: &'a str, children: Vec<OptionnalFiber<'a>>) -> Fiber<'a> {
    match children[0] {
        Some(child) => Fiber {
            type_,
            child: Some(&child),
        },
        None => Fiber { type_, child: None },
    }
}

fn create_markup(fiber: &Fiber) -> String {
    let mut markup = String::from(format!("<{}>\n", fiber.type_));
    if let Some(child) = fiber.child {
        markup.push_str(create_markup(child).as_str())
    }
    markup
}

fn render<'a>(fiber: &'a Fiber) -> String {
    create_markup(fiber)
}

fn main() {
    let span = create_fiber("span", vec![None]);
    let p = create_fiber("p", vec![Some(&span)]);
    let section = create_fiber("section", vec![Some(&p)]);

    println!("result is \n{}", render(&section));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let span = create_fiber("span", vec![None]);
        let p = create_fiber("p", vec![Some(&span)]);
        let section = create_fiber("section", vec![Some(&p)]);
        assert_eq!(render(&section), "<section>\n<p>\n<span>\n")
    }
}
