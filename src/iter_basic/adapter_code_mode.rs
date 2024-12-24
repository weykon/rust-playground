// 从Skip的编码上追寻练习这种优雅的编码思维习惯
#[derive(Debug)]
struct Weapon {
    name: String,
    damage: u32,
}

#[derive(Debug)]
struct Armor {
    name: String,
    defense: u32,
}

// 2. 定义能力特征
trait Equipment {
    fn get_name(&self) -> &str;
}

// 为基础装备实现特征
impl Equipment for Weapon {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Equipment for Armor {
    fn get_name(&self) -> &str {
        &self.name
    }
}

// 3. 创建增强适配器
// 火焰附魔
struct FireEnchanted<E> {
    equipment: E,
    fire_damage: u32,
}

// 冰霜附魔
struct FrostEnchanted<E> {
    equipment: E,
    frost_damage: u32,
}

// 4. 为增强适配器实现装备特征
impl<E: Equipment> Equipment for FireEnchanted<E> {
    fn get_name(&self) -> &str {
        self.equipment.get_name()
    }
}

impl<E: Equipment> Equipment for FrostEnchanted<E> {
    fn get_name(&self) -> &str {
        self.equipment.get_name()
    }
}


// 7. 使用示例
fn main() {
    // 创建基础武器
    let sword = Weapon {
        name: String::from("铁剑"),
        damage: 10,
    };

    // 添加多重附魔
    let epic_sword = sword
        .add_fire()  // 添加火焰附魔
        .add_frost(); // 再添加冰霜附魔

    // 以上相当于创建了 FrostEnchanted<FireEnchanted<Weapon>>

    print_equipment_info(&epic_sword);
}

// 8. 辅助函数来打印装备信息
fn print_equipment_info(equipment: &impl Equipment) {
    println!("装备名称: {}", equipment.get_name());
}

// 9. 添加更多有趣的增强器
struct GlowingEffect<E> {
    equipment: E,
    glow_color: String,
}

impl<E: Equipment> Equipment for GlowingEffect<E> {
    fn get_name(&self) -> &str {
        self.equipment.get_name()
    }
}
trait EnchantmentExt: Equipment + Sized {
    fn add_fire(self) -> FireEnchanted<Self>;
    fn add_frost(self) -> FrostEnchanted<Self>;
    fn add_glow(self, color: &str) -> GlowingEffect<Self>;
}
// 10. 扩展增强特征
impl<T: Equipment> EnchantmentExt for T {
    fn add_fire(self) -> FireEnchanted<Self> {
        FireEnchanted {
            equipment: self,    // 将原始装备包装进去
            fire_damage: 10,    // 添加新属性
        }
    }

    fn add_frost(self) -> FrostEnchanted<Self> {
            FrostEnchanted {
                equipment:  self,
                frost_damage: 8,
            }
    }

    fn add_glow(self, color: &str) -> GlowingEffect<Self> {
        GlowingEffect {
            equipment: self,
            glow_color: "red".to_string(),
        }
    }

}

// 11. 更复杂的组合示例
fn create_epic_weapon() {
    let basic_sword = Weapon {
        name: String::from("新手剑"),
        damage: 5,
    };

    let epic_sword = basic_sword
        .add_fire()
        .add_frost()
        .add_glow("紫色");

    // 现在我们有了一把 带有紫色光效的霜火双重附魔新手剑！
}
