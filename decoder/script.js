boxes = [document.getElementById("sgn0"),
document.getElementById("exp2"),
document.getElementById("exp1"),
document.getElementById("exp0"),
document.getElementById("man1"),
document.getElementById("man2"),
document.getElementById("man3"),
document.getElementById("man4")];

function get_bit_value(i) {
    return boxes[i].checked ? 1 : 0;
}

function update_value() {
    sgn = get_bit_value(0);
    exp = 4 * get_bit_value(1) + 2 * get_bit_value(2) + get_bit_value(3) - 3;
    man = 1 + get_bit_value(4) * 0.5 + get_bit_value(5) * 0.25 + get_bit_value(6) * 0.125 + get_bit_value(7) * 0.0625;
    console.log(sgn, exp, man);
    if (exp == -3 && man == 1) {
        value = ((-1) ** sgn) * 0.;
    } else {
        value = ((-1) ** sgn) * man * (2 ** exp);
    }
    document.getElementById("value").innerHTML = "Value: " + value;
}

boxes.forEach(_ => {
    addEventListener("change", update_value)
});