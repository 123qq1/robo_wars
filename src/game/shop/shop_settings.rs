pub fn get_json() -> String{
    r#"
        [
            {"price": 55, "available": true, "product": {"name": "One","finish": 20, "progress": 0, "speed": 0.2, "product": {"name":"One One", "health": 5.0, "speed": 0.35, "dmg": 1.0, "range": 3.0}}   },
            {"price": 45, "available": true, "product": {"name": "Two","finish": 25, "progress": 0, "speed": 0.4, "product": {"name":"One Two", "health": 6.0, "speed": 0.3, "dmg": 2.0, "range": 1.0}}   }
        ]
        "#.to_string()
}
