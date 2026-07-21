use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{Rglica, editor::EditedCallback};

pub struct Editor<'a, Obj: EditedCallback, Field> {
    obj:       Rglica<Obj>,
    field:     Rglica<Field>,
    _lifetime: PhantomData<&'a Obj>,
}

impl<'a, Obj: EditedCallback, Field> Editor<'a, Obj, Field> {
    pub fn new(obj: &'a Obj, field: &'a Field) -> Self {
        Self {
            obj:       Rglica::from_ref(obj),
            field:     Rglica::from_ref(field),
            _lifetime: PhantomData,
        }
    }
}

impl<Obj: EditedCallback, Field> Deref for Editor<'_, Obj, Field> {
    type Target = Field;
    fn deref(&self) -> &Self::Target {
        self.field.deref()
    }
}

impl<Obj: EditedCallback, Field> DerefMut for Editor<'_, Obj, Field> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.field.deref_mut()
    }
}

impl<Obj: EditedCallback, Field> Drop for Editor<'_, Obj, Field> {
    fn drop(&mut self) {
        self.obj.edited();
    }
}
