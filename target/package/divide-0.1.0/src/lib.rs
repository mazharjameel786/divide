#![allow(dead_code)]
#![allow(unused_variables)]

pub mod arithmetic{

   pub mod division {
        
        pub fn division_res(x:f64,y:f64)->Option<f64>{

            if y==0.0{
                    None
            }else{
                Some(x/y)
            }
        }
    }

}
