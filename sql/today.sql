-- setup schema
CREATE TABLE log(
  "when" DATETIME,
  "toggle" TEXT,
  "activity" TEXT
);

-- import and support csv
.mode csv
.import ../db/db.csv log

-- SELECT MAX("when"),"toggle", "activity"
--   FROM log WHERE "toggle"=="stop"
--   GROUP BY "activity" ORDER BY "when" ;

-- SELECT "when","toggle", "activity" FROM log WHERE "toggle" == "start" UNION SELECT * FROM log ;
--

Create table starts as SELECT "when", "toggle", "activity"
  FROM log WHERE "toggle"=="start" AND "when" > date('now','-1 day');

CREATE TABLE stops as  SELECT "when", "toggle", "activity"
    FROM log WHERE "toggle"=="stop" AND "when" > date('now', '-1 day');

CREATE TABLE complete as SELECT max(start) as start, min(stop) as stop, activity FROM (
  SELECT max(starts."when") AS start, min(stops."when") AS stop, starts."activity" AS activity
    FROM starts FULL JOIN stops ON starts."when" < stops."when"
  WHERE (starts."activity" == stops."activity") OR stops."activity" == "all" or stops.activity == Null
  GROUP BY starts."when"
) GROUP BY stop ORDER BY stop desc;

CREATE TABLE incomplete AS
  SELECT starts."when" as start, starts."activity" as activity FROM starts
    WHERE starts."when" NOT IN (SELECT start FROM complete)
      AND starts."when" > (Select max(stops."when") FROM stops WHERE stops."activity"="all");

SELECT activity, timediff(SUM(JULIANDAY(complete.stop) - JULIANDAY(complete.start)),0) as total
  FROM complete GROUP BY activity ORDER BY Max(complete.stop) DESC;
