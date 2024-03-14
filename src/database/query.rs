pub const SELECT_USER_INFOMATION: &str = r#"
            select name,login,email,id
            from users
        "#;
pub const UPDADE_USER_INFORMATION: &str = r#"
            update users
            set name=?1, email=?2 
            where login=?3
        "#;
pub const SELECT_COMPUTER_INFORMATION_WITH_LAST_USER: &str = r#"
        select serialnumber ,brands.name as brand, model, login as actual_user
        from computer 
        join brands on computer.brand  = brands.id 
        --left join has on has.computer_id = computer.id 
        left join (
        	select *
        	from has
        	ORDER by date_begin
        	desc
        	LIMIT 1
        ) as last_user on last_user.computer_id = computer.id
        left join users on users.id = last_user.user_id   
                "#;

pub const UPDATE_LAST_USER_COMPUTER: &str = r#"
            update has
            set date_end=?1 
            where user_id=(    
                        select id 
                    	from users u 
                    	WHERE u.login =?2        
                        )
            and computer_id=(
                     	SELECT id 
                    	from computer c 
                    	WHERE c.serialnumber = ?3
                        )
            and date_end is NULL
        "#;
pub const INSERT_NEW_USER_COMPUTER: &str = r#"
        insert into has (computer_id, user_id, date_begin)
        values (
        (select id from computer WHERE serialnumber = ?1),
        ?2,
        ?3
        )   
        "#;
