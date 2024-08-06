// #![allow(dead_code)]
pub const SELECT_USER_INFOMATION: &str = r#"
            SELECT name,login,email,id,document,department,phone_number,extension
            FROM users
            order by name
        "#;
pub const SELECT_SPECIFIC_USER_INFOMATION: &str = r#"
            SELECT name,login,email,id,document,department,phone_number,extension
            FROM users
            where login =?1
        "#;
pub const SELECT_SPECIFIC_EQUIPAMENT_MODEL_INFOMATION: &str = r#"
        SELECT name,
        	(select name from brands where id=equipament_model.brand) as brand,
        	(select name from cpu where id=equipament_model.cpu) as cpu,
        	(select name from gpu where id=equipament_model.gpu) as gpu
        FROM equipament_model
        WHERE name=?1
"#;

pub const SELECT_COMPUTER_INFORMATION_WITH_LAST_USER: &str = r#"
SELECT * FROM (
    SELECT 
        equipaments.serialnumber,
        brands.name AS brand,
		equipaments.memory,
		equipaments.storage,
		equipaments.observation,
		equipament_model.name as 'model',
        login AS actual_user,
        ROW_NUMBER() OVER (PARTITION BY equipaments.id ORDER BY has.date_begin DESC) AS rn
    FROM 
        equipaments
	JOIN equipament_model ON equipaments.model= equipament_model.id
    JOIN brands ON equipament_model.brand = brands.id
	LEFT JOIN has ON has.computer_id = equipaments.id
    LEFT JOIN users ON users.id = has.user_id
    WHERE 
        has.date_end IS NULL
) AS sub
WHERE 
    sub.rn = 1;
"#;

pub const SELECT_BRAND: &str = r#"
    SELECT id,name
    FROM brands 
    order by name
"#;

pub const SELECT_PHONE_NUMBER: &str = r#"
    SELECT id,name
    FROM phone_numbers
    "#;

pub const SELECT_DEPARTMENT: &str = r#"
    SELECT id,name
    FROM departments
    order by name
"#;

pub const SELECT_DEPARTMENT_BY_ID: &str = r#"
    SELECT id,name
    FROM departments
    where id=?1
"#;

pub const SELECT_DEPARTMENT_BY_NAME: &str = r#"
    SELECT id
    from departments
    where name = ?1
"#;

pub const SELECT_CPU: &str = r#"
    SELECT cpu.name,brands.name as brand
    FROM cpu
    JOIN brands ON cpu.brand = brands.id
    order by cpu.name
"#;

pub const SELECT_BRAND_BY_ID: &str = r#"
    SELECT id,name
    FROM brands
    where id=?1
"#;

pub const SELECT_CPU_BY_NAME: &str = r#"
    SELECT id
    from cpu
    where name = ?1
"#;
pub const SELECT_CPU_BY_ID: &str = r#"
    SELECT id,name
    FROM cpu 
    where id=?1
"#;

pub const SELECT_BRAND_BY_NAME: &str = r#"
    SELECT id
    from brands
    where name = ?1
"#;
pub const SELECT_GPU_BY_ID: &str = r#"
    SELECT id,name
    FROM gpu
    where id=?1
"#;

pub const SELECT_GPU_BY_NAME: &str = r#"
    SELECT id
    from gpu   
    where name = ?1
"#;
pub const SELECT_EQUIPAMENT_MODEL: &str = r#"
    SELECT equipament_model.name,brands.name as brand,cpu.name as cpu,gpu.name as gpu
    FROM equipament_model
    JOIN cpu ON equipament_model.cpu = cpu.id
    JOIN gpu On equipament_model.gpu = gpu.id
    JOIN brands ON equipament_model.brand = brands.id
"#;

pub const SELECT_GPU: &str = r#"
    SELECT gpu.name,brands.name as brand
    FROM gpu
    JOIN brands ON gpu.brand = brands.id
    order by gpu.name
"#;
pub const INSERT_GPU: &str = r#"
   INSERT INTO gpu (name,brand) 
    VALUES (
    ?1,
        (SELECT id
        FROM brands
        WHERE name = ?2)
)
"#;

pub const UPDADE_USER_INFORMATION: &str = r#"
    UPDATE users
    SET name=?1, 
        email=?2,
        phone_number=?3,
        department=?4,
        extension=?5
    WHERE login=?3
"#;
pub const UPDADE_EQUIPAMENT_MODEL_INFORMATION: &str = r#"
    UPDATE equipament_model
    SET name=?1, 
        brand= (SELECT id FROM brands WHERE name=?2),
        cpu=   (SELECT id FROM cpu WHERE name=?3),
        gpu=   (SELECT id FROM GPU WHERE name=?4)
    WHERE name=?5
"#;
pub const INSERT_CPU: &str = r#"
   INSERT INTO cpu (name,brand) 
    VALUES (
    ?1,
        (SELECT id
        FROM brands
        WHERE name = ?2)
)
"#;

pub const INSERT_USER_INFORMATION: &str = r#"
    insert into users (name,department,document,email,login,extension,phone_number)
    VALUES (
            ?1,
            (select id from departments where name = ?2),
            ?3,
            ?4,
            ?5,
            ?6,
            ?7
            
    )    
"#;
pub const INSERT_COMPUTER: &str = r#"
   INSERT INTO equipaments(serialnumber,storage,memory,model,observation) 
    VALUES (
    ?1,
    ?2,
    ?3,
    (SELECT id FROM equipament_model WHERE name = ?4),
    ?5
)

"#;
pub const UPDATE_LAST_USER_COMPUTER: &str = r#"
            update has
            set date_end=?1 
            where user_id=(    
                        SELECT id 
                    	FROM users u 
                    	WHERE u.login =?2        
                        )
            and computer_id=(
                     	SELECT id 
                    	FROM equipaments c 
                    	WHERE c.serialnumber = ?3
                        )
            AND date_end IS NULL
        "#;
pub const INSERT_NEW_USER_COMPUTER: &str = r#"
        INSERT INTO has (computer_id, user_id, date_begin)
        VALUES (
        (SELECT id FROM equipaments WHERE serialnumber = ?1),
        (SELECT id FROM users WHERE login like ?2),
        ?3
        )   
        "#;

pub const INSERT_BRAND: &str = r#"
   INSERT INTO brands (name) 
    VALUES (?1)
"#;

pub const DELETE_BRAND: &str = r#"
   DELETE from brands
    where name = ?1 
"#;

pub const INSERT_PHONE_NUMBER: &str = r#"
   INSERT INTO phone_numbers(name) 

    VALUES (?1)
"#;

pub const DELETE_PHONE_NUMBER: &str = r#"
   DELETE from phone_numbers
    where name = ?1 
"#;

pub const INSERT_DEPARTMENT: &str = r#"
   INSERT INTO departments (name) 
    VALUES (?1)
"#;

pub const DELETE_DEPARTMENT: &str = r#"
   DELETE from departments
    where name = ?1 
"#;

pub const INSERT_EQUIPAMENT_MODEL: &str = r#"
   INSERT INTO equipament_model (name,brand,cpu,gpu) 
    VALUES (
    ?1,
        (SELECT id
        FROM brands
        WHERE name = ?2),
        
        (SELECT id
        FROM cpu
        WHERE name = ?3),
        
        (SELECT id
        FROM gpu
        WHERE name = ?4)
    )
"#;

pub const DELETE_EQUIPAMENT_MODEL: &str = r#"
   DELETE from equipament_model    
   where name = ?1 
"#;

pub const DELETE_GPU: &str = r#"
   DELETE from gpu
    where name = ?1 
"#;
pub const DELETE_CPU: &str = r#"
   DELETE from cpu
    where name = ?1 
"#;
