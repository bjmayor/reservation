-- if user_id is null, find all reservations within during for the resource
-- if resource_id is null, find all reservations within during for the user
-- if both are null, find all reservations within during
-- if both set, find all reservations within during for the resource and user
CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during TSTZRANGE) RETURNS TABLE (like rsvp.reservations) AS $$
begin
-- if user_id is null, find all reservations within during for the resource
    if uid is null and rid is null then
        return query select * from rsvp.reservations where  during @> timespan;
    elseif uid is null then
        return query select * from rsvp.reservations where resource_id = rid and during @> timespan;
    elseif rid is null then
-- if resource_id is null, find all reservations within during for the user
        return query select * from rsvp.reservations where user_id = uid and during @> timespan;
    else
-- if both set, find all reservations within during for the resource and user
        return query select * from rsvp.reservations where user_id = uid and resource_id = rid and during @> timespan;
    end if;
    end;
    $$ LANGUAGE plpgsql;
