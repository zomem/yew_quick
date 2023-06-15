/// 检查是否有单位,和计算
pub fn is_have_unit(data: &str) -> bool {
    // px pt pc rem em mm in vw vh calc
    if data.contains("p") {
        return true;
    }
    if data.contains("e") {
        return true;
    }
    if data.contains("v") {
        return true;
    }
    if data.contains("m") {
        return true;
    }
    if data.contains("c") {
        return true;
    }
    if data.contains("i") {
        return true;
    }
    false
}

/// 将 + - * / 两边加上空格
pub fn add_op_space(data: &str) -> String {
    if data.contains("+") {
        let new_data = data.replace("+", " + ");
        return new_data.to_string();
    }
    if data.contains("-") {
        let new_data = data.replace("-", " - ");
        return new_data.to_string();
    }
    if data.contains("*") {
        let new_data = data.replace("*", " * ");
        return new_data.to_string();
    }
    if data.contains("/") {
        let new_data = data.replace("/", " / ");
        return new_data.to_string();
    }
    data.to_string()
}
