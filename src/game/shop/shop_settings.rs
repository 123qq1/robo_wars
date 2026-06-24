pub fn get_json() -> String{
    r#"
        [
            {"price": 55, "available": true, "product": {"name": "One","finish": 80, "progress": 0, "speed": 0.2, "product": {"name":"One One", "health": 200.0, "speed": 0.35, "dmg": 3.0, "range": 60.0}}   },
            {"price": 45, "available": true, "product": {"name": "Two","finish": 100, "progress": 0, "speed": 0.4, "product": {"name":"One Two", "health": 600.0, "speed": 0.3, "dmg": 5.0, "range": 5.0}}   }
        ]
        "#.to_string()
}
