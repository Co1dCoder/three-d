use gl;
use crate::static_mesh::StaticMesh;
pub use std::slice::Iter;

#[derive(Debug)]
pub enum Error {
    AttributeNotFound {message: String}
}

pub struct Att {
    pub name: String,
    pub no_components: usize
}

pub struct VertexBuffer {
    gl: gl::Gl,
    id: u32,
    stride: usize,
    attributes_infos: Vec<Att>
}

impl VertexBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<VertexBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = VertexBuffer{gl: gl.clone(), id, stride:0, attributes_infos: Vec::new() };
        buffer.bind();
        Ok(buffer)
    }

    pub fn bind(&self)
    {
        bind(&self.gl, self.id, gl::ARRAY_BUFFER);
    }

    pub fn stride(&self) -> usize
    {
        self.stride
    }

    pub fn attributes_iter(&self) -> Iter<Att>
    {
        self.attributes_infos.iter()
    }

    pub fn fill_from_attributes(&mut self, mesh: &StaticMesh, attribute_names: &Vec<&str>) -> Result<(), Error>
    {
        self.attributes_infos = Vec::new();
        self.stride = 0;
        for attribute_name in attribute_names {
            self.stride = self.stride + mesh.attribute(attribute_name).ok_or(
                    Error::AttributeNotFound {message: format!("The attribute {} is needed for rendering but is not found in mesh.", attribute_name)}
                )?.no_components;
        }

        let no_vertices = mesh.no_vertices();
        let mut data: Vec<f32> = vec![0.0; self.stride * no_vertices];
        let mut offset = 0;
        for name in attribute_names.iter()
        {
            let attribute = mesh.attribute(name).unwrap();
            let no_components = attribute.no_components;
            self.attributes_infos.push(Att {name: name.to_string(), no_components});
            let mut index = offset;
            for i in 0..no_vertices {
                for j in 0..no_components {
                    data[index + j] = attribute.data[i * no_components + j];
                }
                index += self.stride;
            }
            offset = offset + no_components;
        }

        self.fill_with(data);
        Ok(())
    }

    pub fn fill_with(&mut self, data: Vec<f32>)
    {
        self.bind();
        unsafe {
            self.gl.BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
        }
    }
}


pub struct ElementBuffer {
    gl: gl::Gl,
    id: u32,
}

impl ElementBuffer
{
    pub fn create(gl: &gl::Gl) -> Result<ElementBuffer, Error>
    {
        let mut id: u32 = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }
        let buffer = ElementBuffer{gl: gl.clone(), id };
        bind(&buffer.gl, buffer.id, gl::ELEMENT_ARRAY_BUFFER);
        Ok(buffer)
    }

    pub fn fill_with(&self, data: &[u32])
    {
        bind(&self.gl, self.id, gl::ELEMENT_ARRAY_BUFFER);
        unsafe {
            self.gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW // usage
            );
        }
    }
}



fn bind(gl: &gl::Gl, id: u32, buffer_type: u32)
{
    unsafe {
        static mut CURRENTLY_USED: u32 = std::u32::MAX;
        if id != CURRENTLY_USED
        {
            gl.BindBuffer(buffer_type, id);
            CURRENTLY_USED = id;
        }
    }
}
