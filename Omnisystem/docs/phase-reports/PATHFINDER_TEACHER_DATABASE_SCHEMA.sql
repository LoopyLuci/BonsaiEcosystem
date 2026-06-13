-- PATHFINDER Teacher Service Database Schema
-- Tables for classroom management, student rosters, and alerts

-- ============================================================================
-- CLASSROOM TABLES
-- ============================================================================

-- Classrooms created by teachers
CREATE TABLE IF NOT EXISTS classrooms (
  id UUID PRIMARY KEY,
  teacher_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  subject VARCHAR(100),
  grade_level VARCHAR(20),
  capacity INTEGER DEFAULT 30,
  invite_code VARCHAR(20) UNIQUE NOT NULL,
  settings JSONB DEFAULT '{
    "allow_peer_learning": true,
    "show_leaderboard": true,
    "parent_access": true,
    "mastery_threshold": 0.85
  }',
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_classrooms_teacher_id (teacher_id),
  INDEX idx_classrooms_invite_code (invite_code),
  UNIQUE (id, teacher_id)
);

-- Student memberships in classrooms
CREATE TABLE IF NOT EXISTS classroom_students (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,
  student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(classroom_id, student_id),
  INDEX idx_classroom_students_classroom (classroom_id),
  INDEX idx_classroom_students_student (student_id)
);

-- ============================================================================
-- INTERVENTION ALERT TABLES
-- ============================================================================

-- Automated alerts for struggling students
CREATE TABLE IF NOT EXISTS intervention_alerts (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,
  student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  alert_type VARCHAR(50) NOT NULL, -- 'struggling', 'falling_behind', 'inactive'
  skill_id UUID REFERENCES skills(id),
  message TEXT NOT NULL,
  severity VARCHAR(20) DEFAULT 'medium', -- 'low', 'medium', 'high'
  p_know DECIMAL(3, 2), -- Probability of knowing this skill
  days_since_progress INTEGER,
  recommendation TEXT,
  resolved BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  resolved_at TIMESTAMP,
  dismissed_at TIMESTAMP,
  INDEX idx_alerts_classroom (classroom_id),
  INDEX idx_alerts_student (student_id),
  INDEX idx_alerts_severity (severity),
  INDEX idx_alerts_resolved (resolved),
  INDEX idx_alerts_created (created_at DESC)
);

-- ============================================================================
-- TEACHER STATISTICS TABLES
-- ============================================================================

-- Daily classroom statistics (cached for performance)
CREATE TABLE IF NOT EXISTS classroom_daily_stats (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,
  stat_date DATE NOT NULL,
  total_students INTEGER,
  active_students INTEGER,
  exercises_completed INTEGER,
  avg_mastery DECIMAL(3, 2),
  new_masteries INTEGER,
  struggling_students INTEGER,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(classroom_id, stat_date),
  INDEX idx_classroom_stats_classroom (classroom_id),
  INDEX idx_classroom_stats_date (stat_date)
);

-- Skill performance within a classroom
CREATE TABLE IF NOT EXISTS classroom_skill_stats (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,
  skill_id UUID NOT NULL REFERENCES skills(id),
  avg_mastery DECIMAL(3, 2),
  students_mastered INTEGER,
  students_developing INTEGER,
  students_beginner INTEGER,
  students_struggling INTEGER,
  avg_time_to_mastery_days INTEGER,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(classroom_id, skill_id),
  INDEX idx_classroom_skill_stats_classroom (classroom_id),
  INDEX idx_classroom_skill_stats_skill (skill_id)
);

-- ============================================================================
-- PARENT LINKING TABLES (Week 6)
-- ============================================================================

-- Parent/Guardian account linking to students
CREATE TABLE IF NOT EXISTS parent_student_links (
  id UUID PRIMARY KEY,
  parent_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  relationship VARCHAR(50), -- 'parent', 'guardian', 'relative'
  verified BOOLEAN DEFAULT FALSE,
  verification_code VARCHAR(100),
  verification_sent_at TIMESTAMP,
  verified_at TIMESTAMP,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(parent_id, student_id),
  INDEX idx_parent_links_parent (parent_id),
  INDEX idx_parent_links_student (student_id),
  INDEX idx_parent_links_verified (verified)
);

-- ============================================================================
-- NOTIFICATION PREFERENCE TABLES (Week 6)
-- ============================================================================

-- User notification preferences
CREATE TABLE IF NOT EXISTS notification_preferences (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  notify_mastery BOOLEAN DEFAULT TRUE,
  notify_alerts BOOLEAN DEFAULT TRUE,
  notify_daily_summary BOOLEAN DEFAULT TRUE,
  notify_weekly_report BOOLEAN DEFAULT FALSE,
  notify_achievements BOOLEAN DEFAULT TRUE,
  email_frequency VARCHAR(20) DEFAULT 'daily', -- 'immediate', 'daily', 'weekly', 'never'
  quiet_hours_enabled BOOLEAN DEFAULT FALSE,
  quiet_hours_start TIME,
  quiet_hours_end TIME,
  timezone VARCHAR(50),
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(user_id),
  INDEX idx_notification_prefs_user (user_id)
);

-- Notification delivery log
CREATE TABLE IF NOT EXISTS notifications_sent (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  notification_type VARCHAR(50), -- 'mastery', 'alert', 'summary', 'achievement'
  channel VARCHAR(20), -- 'email', 'push', 'sms'
  subject VARCHAR(255),
  message TEXT,
  delivered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  opened_at TIMESTAMP,
  clicked_at TIMESTAMP,
  INDEX idx_notifications_user (user_id),
  INDEX idx_notifications_type (notification_type),
  INDEX idx_notifications_channel (channel),
  INDEX idx_notifications_delivered (delivered_at DESC)
);

-- ============================================================================
-- ACHIEVEMENT TABLES (Week 7)
-- ============================================================================

-- Achievement definitions
CREATE TABLE IF NOT EXISTS achievements (
  id UUID PRIMARY KEY,
  title VARCHAR(100) NOT NULL,
  description TEXT,
  icon_url VARCHAR(255),
  unlock_condition JSONB, -- {"type": "mastery", "threshold": 0.85, "skill_id": "..."}
  reward_points INTEGER DEFAULT 10,
  rarity VARCHAR(20) DEFAULT 'common', -- 'common', 'rare', 'epic', 'legendary'
  category VARCHAR(50), -- 'speed', 'accuracy', 'consistency', 'milestone'
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX idx_achievements_rarity (rarity),
  INDEX idx_achievements_category (category)
);

-- Student achievement unlocks
CREATE TABLE IF NOT EXISTS learner_achievements (
  id UUID PRIMARY KEY,
  learner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  achievement_id UUID NOT NULL REFERENCES achievements(id),
  unlocked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(learner_id, achievement_id),
  INDEX idx_learner_achievements_learner (learner_id),
  INDEX idx_learner_achievements_achievement (achievement_id),
  INDEX idx_learner_achievements_unlocked (unlocked_at DESC)
);

-- ============================================================================
-- CURRICULUM PREFERENCE TABLES (Week 7)
-- ============================================================================

-- Student learning preferences and goals
CREATE TABLE IF NOT EXISTS learner_preferences (
  id UUID PRIMARY KEY,
  learner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  difficulty_level VARCHAR(20) DEFAULT 'intermediate', -- 'beginner', 'intermediate', 'advanced'
  learning_pace VARCHAR(20) DEFAULT 'moderate', -- 'slow', 'moderate', 'fast'
  preferred_exercise_types JSONB, -- ["multiple_choice", "translation", "listening"]
  learning_goals JSONB, -- {"primary_goal": "fluency", "timeframe": "6_months"}
  study_time_available_hours_per_week DECIMAL(3, 1),
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(learner_id),
  INDEX idx_learner_prefs_learner (learner_id)
);

-- Recommended learning paths (cached results of adaptive algorithm)
CREATE TABLE IF NOT EXISTS learner_recommended_paths (
  id UUID PRIMARY KEY,
  learner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  recommended_skills JSONB, -- Array of skill IDs in recommended order
  reasoning TEXT,
  calculated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP,
  UNIQUE(learner_id),
  INDEX idx_recommended_paths_learner (learner_id),
  INDEX idx_recommended_paths_expires (expires_at)
);

-- ============================================================================
-- TEACHER INSIGHTS TABLES (Week 8)
-- ============================================================================

-- Cached learning insights for teachers
CREATE TABLE IF NOT EXISTS classroom_insights (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,
  insight_type VARCHAR(50), -- 'cohort_trend', 'skill_bottleneck', 'student_pattern'
  insight_data JSONB,
  priority_level VARCHAR(20), -- 'low', 'medium', 'high'
  action_recommended TEXT,
  generated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP,
  INDEX idx_classroom_insights_classroom (classroom_id),
  INDEX idx_classroom_insights_type (insight_type),
  INDEX idx_classroom_insights_priority (priority_level)
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Performance indexes for common queries
CREATE INDEX IF NOT EXISTS idx_classroom_students_joined
  ON classroom_students(joined_at DESC);

CREATE INDEX IF NOT EXISTS idx_intervention_alerts_classroom_unresolved
  ON intervention_alerts(classroom_id, resolved)
  WHERE resolved = FALSE;

CREATE INDEX IF NOT EXISTS idx_intervention_alerts_severity_created
  ON intervention_alerts(severity, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_classroom_daily_stats_date_desc
  ON classroom_daily_stats(classroom_id, stat_date DESC);

-- ============================================================================
-- MATERIALIZED VIEWS FOR ANALYTICS (Optional, for performance)
-- ============================================================================

-- Classroom overview (cached)
CREATE MATERIALIZED VIEW IF NOT EXISTS classroom_overview AS
SELECT
  c.id,
  c.teacher_id,
  c.name,
  c.subject,
  c.grade_level,
  COUNT(DISTINCT cs.student_id) as total_students,
  COUNT(DISTINCT CASE WHEN ea.created_at > CURRENT_TIMESTAMP - INTERVAL '7 days'
                       THEN cs.student_id END) as active_students,
  AVG(COALESCE(lp.mastery_percentage, 0)) as avg_mastery,
  COUNT(DISTINCT CASE WHEN ia.resolved = FALSE
                       THEN ia.id END) as unresolved_alerts,
  c.created_at,
  CURRENT_TIMESTAMP as view_updated_at
FROM classrooms c
LEFT JOIN classroom_students cs ON c.id = cs.classroom_id
LEFT JOIN users u ON cs.student_id = u.id
LEFT JOIN learner_progress lp ON cs.student_id = lp.user_id
LEFT JOIN exercise_attempts ea ON cs.student_id = ea.user_id
LEFT JOIN intervention_alerts ia ON c.id = ia.classroom_id
GROUP BY c.id, c.teacher_id, c.name, c.subject, c.grade_level, c.created_at;

-- Create index on materialized view
CREATE INDEX IF NOT EXISTS idx_classroom_overview_teacher
  ON classroom_overview(teacher_id);

-- ============================================================================
-- TRIGGERS FOR ALERTS (Optional, for automation)
-- ============================================================================

-- Auto-generate alerts when student shows signs of struggle
-- Note: This would be implemented as a background job for better performance
-- but can be implemented as a trigger in the database

CREATE OR REPLACE FUNCTION detect_struggling_students()
RETURNS TRIGGER AS $$
BEGIN
  -- Check if learner's mastery drops below 30% or hasn't improved in 7 days
  INSERT INTO intervention_alerts (
    id, classroom_id, student_id, alert_type, skill_id,
    message, severity, p_know, days_since_progress, recommendation
  )
  SELECT
    gen_random_uuid(),
    cs.classroom_id,
    cs.student_id,
    'struggling',
    s.id,
    'Student is struggling with ' || s.skill_name,
    'high',
    NEW.p_know,
    EXTRACT(DAY FROM CURRENT_TIMESTAMP - (
      SELECT MAX(created_at) FROM exercise_attempts
      WHERE user_id = cs.student_id AND skill_id = s.id
    ))::INTEGER,
    'Consider additional practice or tutoring'
  FROM classroom_students cs
  JOIN skills s ON s.id = NEW.skill_id
  WHERE cs.student_id = NEW.user_id
    AND NEW.p_know < 0.30
  ON CONFLICT DO NOTHING;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Note: Trigger would be created, but commented out for now
-- CREATE TRIGGER trigger_detect_struggling
--   AFTER UPDATE ON learner_skill_states
--   FOR EACH ROW
--   EXECUTE FUNCTION detect_struggling_students();

-- ============================================================================
-- SEED DATA (Optional - for testing)
-- ============================================================================

-- Sample achievements
INSERT INTO achievements (id, title, description, icon_url, unlock_condition, reward_points, rarity, category)
VALUES
  (gen_random_uuid(), 'First Skill Mastered', 'Master your first skill to 85%', '/icons/trophy.svg',
   '{"type": "mastery", "threshold": 0.85}', 50, 'rare', 'milestone'),
  (gen_random_uuid(), 'Speed Learner', 'Master 3 skills in one week', '/icons/lightning.svg',
   '{"type": "speed", "skills": 3, "days": 7}', 100, 'epic', 'speed'),
  (gen_random_uuid(), 'Consistency King', 'Practice 30 days in a row', '/icons/fire.svg',
   '{"type": "streak", "days": 30}', 150, 'epic', 'consistency'),
  (gen_random_uuid(), 'Perfect Score', 'Get 100% on an exercise', '/icons/perfect.svg',
   '{"type": "accuracy", "threshold": 1.0}', 25, 'common', 'accuracy');

-- Notification preferences defaults (would be inserted when user creates account)
-- See USER service for user account creation with notification preference defaults

COMMIT;
