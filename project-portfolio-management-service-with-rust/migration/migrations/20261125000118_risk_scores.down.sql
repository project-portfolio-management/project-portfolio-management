-- 20261125000118_risk_scores: undo the up file beside it.

DROP TRIGGER IF EXISTS risk_scores_write ON risks;

DROP FUNCTION IF EXISTS risk_scores_write();

DROP TABLE IF EXISTS risk_scores;

DROP FUNCTION IF EXISTS risk_scores_guard();
