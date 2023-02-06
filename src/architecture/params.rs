pub struct Params {
    pub options: Vec<(&'static str, Option<&'static str>)>,
    pub req_arguments: Vec<&'static str>,
    pub opt_arguments: Vec<&'static str>,
    pub arg_list: Vec<&'static str>,
}

impl Params {

    fn add_option(&mut self, name: &'static str, data: Option<&'static str>) {
        self.options.push((name, data));
    }

    fn add_req_arg(&mut self, arg: &'static str) {
        self.req_arguments.push(arg);
    }

    fn add_other_arg(&mut self, arg: &'static str) {
        self.opt_arguments.push(arg);
    }

    fn add_arg_to_list(&mut self, arg: &'static str) {
        self.arg_list.push(arg);
    }
}