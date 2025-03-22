CREATE TABLE needs_ride (
  user_id VARCHAR NOT NULL REFERENCES users(id),
  event_id INT NOT NULL REFERENCES event(id),
  PRIMARY KEY (user_id, event_id)
);

CREATE FUNCTION remove_if_in_needs_ride_rider() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM needs_ride
        INNER JOIN event ON needs_ride.event_id = event.id
        INNER JOIN car ON car.event_jd = car.id
        INNER JOIN rider ON rider.car_id = car.id
        WHERE (rider.rider = NEW.user_id)
    ) THEN
        DELETE FROM needs_ride WHERE user_id = NEW.user_id AND event_id IN (SELECT car.event_id
        FROM rider
        INNER JOIN car
            ON rider.car_id = car.id
        WHERE rider.rider = NEW.user_id);
    END IF;
    RETURN NEW;
END; $$ LANGUAGE plpgsql;

CREATE FUNCTION remove_if_in_needs_ride_driver() RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM needs_ride
        INNER JOIN event ON needs_ride.event_id = event.id
        INNER JOIN car ON car.event_id = car.id
        WHERE (car.driver = NEW.driver)
    ) THEN
        DELETE FROM needs_ride WHERE user_id = NEW.driver AND event_id = NEW.event_id;
    END IF;
    RETURN NEW;
END; $$ LANGUAGE plpgsql;

CREATE TRIGGER trg_PreventRideAssociationRider
BEFORE INSERT OR UPDATE ON rider
FOR EACH ROW
EXECUTE FUNCTION remove_if_in_needs_ride_rider();
END;

-- CREATE TRIGGER trg_PreventRideAssociationDriver
-- BEFORE INSERT OR UPDATE ON car
-- FOR EACH ROW
-- EXECUTE FUNCTION remove_if_in_needs_ride_driver();
-- END;