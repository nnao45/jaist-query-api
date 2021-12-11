with fp as (
    select user_id as fp_user_id, latitude, longitude
    from foot_stamp
             inner join user u2 on foot_stamp.user_id = u2.id
)
   , test1 as (
    select *
    from post p
             inner join fp on p.user_id = fp.fp_user_id
    where fp.latitude between -20 and 30
      and p.user_id between 1 and 20000
      and fp.longitude between -20 and 30
      and p.created_at between '1990-11-01' and '2002-01-01'
)
   , test2 as (
    select *
    from post p
             inner join fp on p.user_id = fp.fp_user_id
    where fp.latitude between -50 and 10
      and p.user_id between 100 and 3000
      and fp.longitude between -20 and 30
      and p.created_at between '1990-11-01' and '2002-01-01'
)
select count(*) from test1, test2;