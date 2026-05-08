DO $$
DECLARE
    v_user_id INT;
BEGIN
    IF EXISTS (SELECT 1 FROM users WHERE username = 'test') THEN
        RAISE NOTICE 'test user already exists, skipping';
        RETURN;
    END IF;

    INSERT INTO users (username, motto, look, gender, rank, credits, auth_ticket)
    VALUES ('test', 'Hello Sirius!', 'hd-180-1.ch-215-62', 'M', 7, 1000, 'test_ticket')
    RETURNING id INTO v_user_id;

    INSERT INTO users_stats (user_id, daily_respects, daily_pet_respects)
    VALUES (v_user_id, 3, 3);

    INSERT INTO users_settings (user_id)
    VALUES (v_user_id);

    INSERT INTO users_currency (user_id, currency_type, amount)
    VALUES
        (v_user_id, 0, 500),
        (v_user_id, 5, 100);

    RAISE NOTICE 'test user created with id %', v_user_id;
END;
$$;
