pub fn get_json() -> String{
    r#"
        [
            {"price": 55, "available": true, "product": {"name": "Bow","finish": 80, "progress": 0, "speed": 0.2, "product": {"name":"One One", "health": 200.0, "speed": 0.35, "dmg": 60.0, "range": 60.0, "finish": 60, "progress": 0, "rate": 1.0}}   },
            {"price": 45, "available": true, "product": {"name": "Sword","finish": 100, "progress": 0, "speed": 0.4, "product": {"name":"One Two", "health": 600.0, "speed": 0.3, "dmg": 100.0, "range": 10.0, "finish": 60, "progress": 0, "rate": 1.0}}   },
            {"price": 85, "available": true, "product": {"name": "Brute","finish": 200, "progress": 0, "speed": 0.1, "product": {"name":"One Three", "health": 1500.0, "speed": 0.1, "dmg": 600.0, "range": 5.0, "finish": 60, "progress": 0, "rate": 0.2}}   },
            {"price": 75, "available": true, "product": {"name": "Sniper","finish": 200, "progress": 0, "speed": 0.4, "product": {"name":"One Four", "health": 50.0, "speed": 0.15, "dmg": 800.0, "range": 100.0, "finish": 60, "progress": 0, "rate": 0.1}}   }
        ]
        "#.to_string()
}
