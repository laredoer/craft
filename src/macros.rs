#[macro_export]
macro_rules! map {
  ($key_type:expr, $value_type:expr, {$( $key:expr => $value:expr ),* }) => {
      {
          let mut map_literal = format!("map[{}]{}{{\n", $key_type, $value_type);
          $(
              map_literal.push_str(&format!("  {}: {},\n", $key, $value));
          )*
          map_literal.push_str("}\n");
          map_literal
      }
  };
}

#[macro_export]
macro_rules! import {
  ($(($alias:tt, $path:expr)),*) =>  {
      {
          let mut imports = String::new();
          imports.push_str("import (\n");
          $(
              imports.push_str(&format!("  {} {:?}\n", $alias, $path ));
          )*
          imports.push_str(")\n");
          imports
      }
  };
}

#[macro_export]
macro_rules! package {
    ($name:expr) => {{
        format!("package {}\n", $name)
    }};
}

#[macro_export]
macro_rules! go {
  ( $( $x:expr ),* ) => {
      {
          let mut file = String::new();
          $(
              file.push_str(&format!("{}\n",$x));
          )*
          file
      }
  };
}

#[macro_export]
macro_rules! i18n {
    ($code:expr, $zh_cn:expr, $zh_hk:expr) => {
        format!("i18n.register({}, {}, {});\n", $code, $zh_cn, $zh_hk)
    };
}
