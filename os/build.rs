// 用于生成link_app.S，其读取user/bin下所有程序的名字，链接所有已有的user程序并最终加载

use std::fs::{read_dir, File};
use std::io::{Result, Write};

static TARGET_DIR: &str = "../user/target/riscv64gc-unknown-none-elf/release";

fn main() {
    insert_app_data().unwrap();
}

fn insert_app_data() -> Result<()> {
    let mut file = File::create("src/link_app.S")?;
    let mut apps: Vec<_> = read_dir("../user/src/bin")?
        .into_iter()
        .map(|dir_entry| {
            let mut name = dir_entry.unwrap().file_name().into_string().unwrap();
            name.drain(name.find(".").unwrap()..name.len());
            name
        })
        .collect(); // map切除后缀名，collect消耗迭代器并整合成迭代器对应容器
    apps.sort();

    // 生成代码，写入f
    /*
    生成逻辑：
    */
    writeln!(
        file,
        r#"
  .align 3
  .section .data
  .global _num_app
_num_app:
  .quad {}
  "#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(file, r#"  .quad app_{}_start"#, i)?;
    }

    // 还要加end,中间可以不加是因为无缝衔接
    writeln!(file, r#"  .quad app_{}_end"#, apps.len() - 1)?;

    for (i, name) in apps.iter().enumerate() {
        writeln!(
            file,
            r#"
  .section .data
  .global app_{0}_start
  .global app_{0}_end
app_{0}_start:
  .incbin "{1}{2}.bin"
app_{0}_end:"#,
            i, TARGET_DIR, name
        )?;
    }
    Ok(())
}
