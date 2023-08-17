use pest::Parser;
use pest_derive::Parser;

pub mod ast;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct ComonautParser;

pub fn parse_source(source: &str) -> Result<(), pest::error::Error<Rule>> {
    let mut ast_nodes = vec![];

    let pairs = ComonautParser::parse(Rule::program, source)?;
    for pair in pairs.into_iter() {
        ast_nodes.push(expr_to_ast(pair))
    }

    Ok(())
}

pub fn expr_to_ast(pair: pest::iterators::Pair<Rule>) -> Option<Box<ast::Expression>> {
    match pair.as_rule() {
            Rule::bool => {
                Some(Box::new(ast::Expression::Boolean(pair.as_str() == "true")))
            },
            Rule::decl => {
                let mut pair = pair.into_inner();
                let x = pair.next().unwrap();
                let y = pair.next().unwrap();
                if let Some(exp) = pair.next() {
                    println!("{:?}", exp);
                };
                println!("{:?}",x);
                println!("{:?}",y);
                None
            },
            Rule::integer => {
                let i = pair
                    .as_str()
                    .parse::<i64>()
                    .unwrap();

                Some(Box::new(ast::Expression::Integer(i)))
            },
            Rule::ident =>
                Some(Box::new(ast::Expression::Identifier(pair.as_str().to_string()))),
            Rule::EOI => None,
            _ => unreachable!()
    }
}
