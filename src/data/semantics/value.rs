use data::ast::Unit;

#[derive(Clone, Debug)]
pub enum Value {
	Static(StaticValue),
	Variable(usize, Option<StaticValue>),
	UnrenderedVariable(String),
}

#[derive(Clone, Debug)]
pub enum StaticValue {
	Number(i32, Unit),
	String(String),
	// Color(u8, u8, u8, f64),
}
