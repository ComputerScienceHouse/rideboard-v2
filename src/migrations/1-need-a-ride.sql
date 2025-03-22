CREATE TABLE needs_ride (
    user_id VARCHAR NOT NULL REFERENCES users(id),
    event_id INT NOT NULL REFERENCES event(id),
    PRIMARY KEY (user_id, event_id)
);

CREATE FUNCTION raise_if_already_driver_or_ride() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM car
        WHERE car.driver = NEW.user_id AND car.event_id = NEW.event_id

        UNION ALL

        SELECT 1
        FROM rider
        JOIN car
            ON rider.car_id = car.id
        WHERE rider.rider = NEW.user_id AND car.event_id = NEW.event_id
    ) THEN
        RAISE EXCEPTION 'User % is already a driver or rider for event %', NEW.user_id, NEW.event_id;
    END IF;
    RETURN NEW;
END; $$ LANGUAGE plpgsql;

CREATE TRIGGER trg_PreventRideAssociation
BEFORE INSERT ON needs_ride
FOR EACH ROW
EXECUTE PROCEDURE raise_if_already_driver_or_ride();
END;

CREATE FUNCTION remove_if_in_needs_ride_rider() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM needs_ride
        INNER JOIN event ON needs_ride.event_id = event.id
        INNER JOIN car ON car.event_id = event.id
        WHERE needs_ride.user_id = NEW.rider AND needs_ride.event_id IN (
            SELECT car.event_id
            FROM car
            WHERE car.id = NEW.car_id
        )
    ) THEN
        DELETE FROM needs_ride WHERE user_id = NEW.rider AND event_id IN (
            SELECT car.event_id
            FROM car
            WHERE car.id = NEW.car_id
        );
    END IF;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;

CREATE FUNCTION remove_if_in_needs_ride_driver() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM needs_ride
        INNER JOIN event ON needs_ride.event_id = event.id
        INNER JOIN car ON car.event_id = event.id
        WHERE (car.driver = NEW.driver)
    ) THEN
        DELETE FROM needs_ride WHERE user_id = NEW.driver AND event_id = NEW.event_id;
    END IF;
    RETURN NULL;
END; $$ LANGUAGE plpgsql;

CREATE TRIGGER trg_PreventRideAssociationRider
AFTER INSERT OR UPDATE ON rider
FOR EACH ROW
EXECUTE PROCEDURE remove_if_in_needs_ride_rider();
END;

CREATE TRIGGER trg_PreventRideAssociationDriver
AFTER INSERT OR UPDATE ON car
FOR EACH ROW
EXECUTE PROCEDURE remove_if_in_needs_ride_driver();
END;