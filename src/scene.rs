use crate::object::Object;
use crate::object::ObjectRol;

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn draw_all(&self) {
        for object in &self.objects {
            object.draw();
        }
    }

    pub fn get_object_mut(&mut self, role: ObjectRol) -> Option<&mut Object> {
        self.objects.iter_mut().find(|o| matches! { &o.obj_role , _role })
    }

}
