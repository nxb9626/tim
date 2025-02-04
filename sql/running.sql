.read ./setup.sql

SELECT activity, timediff(SUM(JULIANDAY('now') - JULIANDAY(incomplete.start)),0) as total
  FROM incomplete GROUP BY activity ORDER BY Max(incomplete.start) DESC;
