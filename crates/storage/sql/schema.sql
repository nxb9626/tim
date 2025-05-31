-- the csv representation of the data
CREATE TABLE log(
  "when" DATETIME,
  "toggle" TEXT,
  "activity" TEXT
);

.mode csv
.import ../db/db.csv log -- will be changed for

Create table starts as SELECT "when", "toggle", "activity"
  FROM log WHERE "toggle"=="start";

CREATE TABLE stops as  SELECT "when", "toggle", "activity"
    FROM log WHERE "toggle"=="stop";
