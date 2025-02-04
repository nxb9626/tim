.read ./setup.sql

SELECT activity, timediff(SUM(JULIANDAY(complete.stop) - JULIANDAY(complete.start)),0) as total
  FROM complete GROUP BY activity ORDER BY Max(complete.stop) DESC;
