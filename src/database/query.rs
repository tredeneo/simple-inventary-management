pub const SELECT_USER_INFOMATION: &str = r#"
            SELECT name,login,email,id
            FROM users
        "#;
pub const UPDADE_USER_INFORMATION: &str = r#"
            UPDATE users
            SET name=?1, email=?2 
            WHERE login=?3
        "#;

pub const SELECT_COMPUTER_INFORMATION_WITH_LAST_USER: &str = r#"
SELECT * FROM (
    SELECT 
        computer.serialnumber,
        brands.name AS brand,
        computer.model,
        login AS actual_user,
        ROW_NUMBER() OVER (PARTITION BY computer.id ORDER BY has.date_begin DESC) AS rn
    FROM 
        computer
    JOIN brands ON computer.brand = brands.id
    LEFT JOIN has ON has.computer_id = computer.id
    LEFT JOIN users ON users.id = has.user_id
    WHERE 
        has.date_end IS NULL
) AS sub
WHERE 
    sub.rn = 1;"#;

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
                    	FROM computer c 
                    	WHERE c.serialnumber = ?3
                        )
            AND date_end IS NULL
        "#;
pub const INSERT_NEW_USER_COMPUTER: &str = r#"
        INSERT INTO has (computer_id, user_id, date_begin)
        VALUES (
        (SELECT id FROM computer WHERE serialnumber = ?1),
        ?2,
        ?3
        )   
        "#;

pub const SELECT_BRAND: &str = r#"
    SELECT id,name
    FROM brands 
"#;

pub const INSERT_BRAND: &str = r#"
   INSERT INTO brands (name) 
    VALUES (?1)
"#;

pub const DELETE_BRAND: &str = r#"
   DELETE from brands
    where name = ?1 
"#;
