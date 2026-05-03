pub struct Ctx<'a> {
    pub s: &'a mut String,
    pub i: &'a mut itoa::Buffer,
    pub f: &'a mut ryu::Buffer,
}
