pub fn get_json() -> String{
    r#"
        [
            {"price": 55, "available": true, "product": {"name": "One","finish": 20, "progress": 0, "speed": 1, "product": {"name":"One One", "health": 5.0, "speed": 4.0, "dmg": 1.0, "range": 3.0}}   },
            {"price": 45, "available": false, "product": {"name": "Two","finish": 25, "progress": 0, "speed": 2, "product": {"name":"One Two", "health": 6.0, "speed": 3.0, "dmg": 2.0, "range": 1.0}}   }
        ]
        "#.to_string()
}
