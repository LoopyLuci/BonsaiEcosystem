-- PATHFINDER Learning Platform: PostgreSQL Schema
-- Phase 0-1: Foundation + Core MVP
-- Created: 2026-06-10

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================================
-- USERS & AUTHENTICATION
-- ============================================================================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    email_verified BOOLEAN DEFAULT FALSE,
    email_verified_at TIMESTAMP WITH TIME ZONE,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    language_preference VARCHAR(5) DEFAULT 'en', -- ISO 639-1
    timezone VARCHAR(50) DEFAULT 'UTC',
    is_active BOOLEAN DEFAULT TRUE,
    is_teacher BOOLEAN DEFAULT FALSE,
    is_admin BOOLEAN DEFAULT FALSE,

    -- Privacy & GDPR
    data_export_requested BOOLEAN DEFAULT FALSE,
    data_export_requested_at TIMESTAMP WITH TIME ZONE,
    deletion_requested BOOLEAN DEFAULT FALSE,
    deletion_requested_at TIMESTAMP WITH TIME ZONE,
    deletion_scheduled_at TIMESTAMP WITH TIME ZONE,

    -- Parental consent (COPPA compliance)
    age INTEGER,
    parent_consent BOOLEAN DEFAULT FALSE,
    parent_consent_at TIMESTAMP WITH TIME ZONE,
    parent_email VARCHAR(255),

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_is_active ON users(is_active);
CREATE INDEX idx_users_is_teacher ON users(is_teacher);

-- User sessions (JWT tokens)
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL UNIQUE,
    device_name VARCHAR(100),
    device_type VARCHAR(50), -- 'web', 'ios', 'android'
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    revoked_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_user_sessions_user_id ON user_sessions(user_id);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);

-- ============================================================================
-- SKILLS & CONTENT STRUCTURE
-- ============================================================================

-- Skill ontology (DAG of skills with prerequisites)
CREATE TABLE skills (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) NOT NULL UNIQUE, -- e.g., 'spanish-a1-greetings'
    name VARCHAR(200) NOT NULL,
    description TEXT,
    level VARCHAR(10), -- 'A0', 'A1', 'A2', 'B1', 'B2', 'C1', 'C2'
    language VARCHAR(5), -- ISO 639-1
    category VARCHAR(50), -- 'vocabulary', 'grammar', 'listening', 'reading', 'writing', 'speaking'
    icon_url TEXT,
    color_hex VARCHAR(7),
    estimated_time_minutes INTEGER DEFAULT 30,
    difficulty_level FLOAT DEFAULT 1.0, -- 1.0 = base, 2.0 = double difficulty

    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_skills_code ON skills(code);
CREATE INDEX idx_skills_language ON skills(language);
CREATE INDEX idx_skills_level ON skills(level);
CREATE INDEX idx_skills_category ON skills(category);

-- Prerequisite relationships (DAG edges)
CREATE TABLE skill_prerequisites (
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    prerequisite_skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    is_hard_prerequisite BOOLEAN DEFAULT TRUE, -- If false, just a recommendation
    PRIMARY KEY (skill_id, prerequisite_skill_id)
);

-- Exercises (practice items for a skill)
CREATE TABLE exercises (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,

    type VARCHAR(50) NOT NULL, -- 'multiple_choice', 'translation', 'listening', 'reading', 'writing', 'matching'
    title VARCHAR(255) NOT NULL,
    description TEXT,
    difficulty_delta FLOAT DEFAULT 0.0, -- Relative difficulty from skill base

    -- Multiple choice specific
    prompt TEXT,
    correct_option INT, -- Index of correct option (0-3)
    options TEXT[], -- JSON array of options
    explanation TEXT,

    -- Translation specific
    source_language VARCHAR(5),
    target_language VARCHAR(5),
    source_text TEXT,
    acceptable_answers TEXT[], -- JSON array of acceptable translations

    -- Reading comprehension
    passage TEXT,
    comprehension_questions TEXT, -- JSON array of questions

    -- Listening specific
    audio_url TEXT,
    audio_duration_seconds INT,
    transcript TEXT,

    -- Metadata
    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMP WITH TIME ZONE,
    estimated_time_seconds INT DEFAULT 60,
    usage_count INT DEFAULT 0,
    average_success_rate FLOAT DEFAULT 0.5,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_exercises_skill_id ON exercises(skill_id);
CREATE INDEX idx_exercises_type ON exercises(type);
CREATE INDEX idx_exercises_is_published ON exercises(is_published);

-- Lessons (collections of exercises grouped by skill)
CREATE TABLE lessons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,

    sequence INT NOT NULL, -- Lesson ordering within skill
    title VARCHAR(255) NOT NULL,
    description TEXT,
    learning_objectives TEXT[], -- JSON array

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_lessons_skill_id ON lessons(skill_id);
CREATE UNIQUE INDEX idx_lessons_skill_sequence ON lessons(skill_id, sequence);

-- Exercise ordering within lessons
CREATE TABLE lesson_exercises (
    lesson_id UUID NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    exercise_id UUID NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    sequence INT NOT NULL,
    is_required BOOLEAN DEFAULT TRUE,

    PRIMARY KEY (lesson_id, exercise_id),
    UNIQUE(lesson_id, sequence)
);

-- ============================================================================
-- LEARNER PROGRESS & STATE
-- ============================================================================

-- Learner skill states (Bayesian Knowledge Tracing)
CREATE TABLE learner_skill_states (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,

    -- Bayesian Knowledge Tracing parameters
    p_know FLOAT DEFAULT 0.0, -- Probability of knowledge
    p_slip FLOAT DEFAULT 0.1, -- Probability of slip (know but answer wrong)
    p_guess FLOAT DEFAULT 0.25, -- Probability of guess (don't know but answer right)
    p_transit FLOAT DEFAULT 0.05, -- Probability of transitioning to knowledge

    -- Half-Life Regression (spaced repetition)
    halflife_days FLOAT DEFAULT 21.0, -- Optimal review interval
    last_reviewed_at TIMESTAMP WITH TIME ZONE,
    next_review_at TIMESTAMP WITH TIME ZONE,

    -- Mastery tracking
    is_mastered BOOLEAN DEFAULT FALSE,
    mastery_threshold FLOAT DEFAULT 0.85, -- p_know threshold for mastery
    mastered_at TIMESTAMP WITH TIME ZONE,

    -- Strength (0-1)
    strength FLOAT DEFAULT 0.0,

    -- Practice metrics
    attempt_count INT DEFAULT 0,
    correct_count INT DEFAULT 0,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_learner_skill_states_user_id ON learner_skill_states(user_id);
CREATE INDEX idx_learner_skill_states_next_review_at ON learner_skill_states(next_review_at);
CREATE UNIQUE INDEX idx_learner_skill_states_unique ON learner_skill_states(user_id, skill_id);

-- Exercise attempts (granular practice history)
CREATE TABLE exercise_attempts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    exercise_id UUID NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,

    -- Performance
    was_correct BOOLEAN NOT NULL,
    response TEXT, -- User's answer
    response_time_seconds INT,

    -- Difficulty estimation (for adaptive scheduling)
    exercise_difficulty_rating FLOAT DEFAULT 1.0,
    user_ability_rating FLOAT DEFAULT 0.0,
    discrimination_index FLOAT DEFAULT 0.5,

    -- Feedback
    feedback_given BOOLEAN DEFAULT FALSE,
    feedback_text TEXT,

    attempt_number INT DEFAULT 1, -- 1st, 2nd, 3rd attempt, etc.

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_exercise_attempts_user_id ON exercise_attempts(user_id);
CREATE INDEX idx_exercise_attempts_exercise_id ON exercise_attempts(exercise_id);
CREATE INDEX idx_exercise_attempts_skill_id ON exercise_attempts(skill_id);
CREATE INDEX idx_exercise_attempts_created_at ON exercise_attempts(created_at);
CREATE INDEX idx_exercise_attempts_correct ON exercise_attempts(was_correct);

-- Review history (for spaced repetition scheduling)
CREATE TABLE review_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,

    review_number INT NOT NULL, -- 1st, 2nd, 3rd review
    interval_days INT, -- Days since last review
    ease_factor FLOAT DEFAULT 2.5, -- Supermemo ease factor
    was_success BOOLEAN NOT NULL,
    next_interval_days INT, -- Calculated next interval

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_review_history_user_id ON review_history(user_id);
CREATE INDEX idx_review_history_created_at ON review_history(created_at);

-- ============================================================================
-- LEARNING PATHS & CURRICULUM
-- ============================================================================

-- Curriculum paths (ordered skill sequences)
CREATE TABLE curriculum_paths (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    language VARCHAR(5), -- Target language for language-learning paths
    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Skill ordering within paths
CREATE TABLE curriculum_path_skills (
    curriculum_path_id UUID NOT NULL REFERENCES curriculum_paths(id) ON DELETE CASCADE,
    skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
    sequence INT NOT NULL,
    is_required BOOLEAN DEFAULT TRUE,

    PRIMARY KEY (curriculum_path_id, skill_id),
    UNIQUE(curriculum_path_id, sequence)
);

-- Learner curriculum enrollments
CREATE TABLE learner_curriculum_enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    curriculum_path_id UUID NOT NULL REFERENCES curriculum_paths(id) ON DELETE CASCADE,

    progress_percentage FLOAT DEFAULT 0.0,
    is_completed BOOLEAN DEFAULT FALSE,
    completed_at TIMESTAMP WITH TIME ZONE,

    enrolled_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_curriculum_enrollments_user_id ON learner_curriculum_enrollments(user_id);
CREATE UNIQUE INDEX idx_curriculum_enrollments_unique ON learner_curriculum_enrollments(user_id, curriculum_path_id);

-- ============================================================================
-- TEACHER & CLASSROOM MANAGEMENT
-- ============================================================================

-- Classrooms
CREATE TABLE classrooms (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    teacher_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    code VARCHAR(20) NOT NULL UNIQUE, -- Join code for students
    name VARCHAR(255) NOT NULL,
    description TEXT,

    is_active BOOLEAN DEFAULT TRUE,
    student_count INT DEFAULT 0,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_classrooms_teacher_id ON classrooms(teacher_id);
CREATE INDEX idx_classrooms_code ON classrooms(code);

-- Classroom assignments
CREATE TABLE classroom_assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,

    curriculum_path_id UUID REFERENCES curriculum_paths(id) ON DELETE SET NULL,

    title VARCHAR(255) NOT NULL,
    description TEXT,
    due_date TIMESTAMP WITH TIME ZONE,

    is_published BOOLEAN DEFAULT FALSE,
    published_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_assignments_classroom_id ON classroom_assignments(classroom_id);

-- Student enrollments in classrooms
CREATE TABLE classroom_enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    classroom_id UUID NOT NULL REFERENCES classrooms(id) ON DELETE CASCADE,
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    joined_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_classroom_enrollments_classroom_id ON classroom_enrollments(classroom_id);
CREATE INDEX idx_classroom_enrollments_student_id ON classroom_enrollments(student_id);
CREATE UNIQUE INDEX idx_classroom_enrollments_unique ON classroom_enrollments(classroom_id, student_id);

-- ============================================================================
-- ANALYTICS & METRICS
-- ============================================================================

-- Daily learner metrics (aggregated for privacy)
CREATE TABLE learner_daily_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    metric_date DATE NOT NULL,

    -- Practice metrics
    exercises_attempted INT DEFAULT 0,
    exercises_correct INT DEFAULT 0,
    correct_rate FLOAT DEFAULT 0.0,
    time_spent_seconds INT DEFAULT 0,

    -- Skill progress
    skills_reviewed INT DEFAULT 0,
    new_skills_mastered INT DEFAULT 0,

    -- Streak
    is_streak_day BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_learner_daily_metrics_user_id ON learner_daily_metrics(user_id);
CREATE INDEX idx_learner_daily_metrics_metric_date ON learner_daily_metrics(metric_date);
CREATE UNIQUE INDEX idx_learner_daily_metrics_unique ON learner_daily_metrics(user_id, metric_date);

-- ============================================================================
-- NOTIFICATIONS
-- ============================================================================

-- Smart notifications (ethical, non-addictive)
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    type VARCHAR(50) NOT NULL, -- 'reminder', 'achievement', 'teacher_message'
    title VARCHAR(255) NOT NULL,
    message TEXT,

    -- Reminder context
    skill_id UUID REFERENCES skills(id) ON DELETE SET NULL,
    assignment_id UUID REFERENCES classroom_assignments(id) ON DELETE SET NULL,

    is_read BOOLEAN DEFAULT FALSE,
    read_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_is_read ON notifications(is_read);
CREATE INDEX idx_notifications_created_at ON notifications(created_at);

-- Notification preferences (user control)
CREATE TABLE notification_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    reminder_enabled BOOLEAN DEFAULT TRUE,
    reminder_time TIME DEFAULT '08:00:00', -- Time of day for daily reminders
    reminder_frequency VARCHAR(20) DEFAULT 'daily', -- 'daily', 'weekly', 'never'

    achievement_notifications BOOLEAN DEFAULT TRUE,
    streak_milestones BOOLEAN DEFAULT TRUE,
    teacher_messages BOOLEAN DEFAULT TRUE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_notification_preferences_user_id ON notification_preferences(user_id);

-- ============================================================================
-- CONTENT & FEEDBACK
-- ============================================================================

-- Content feedback (for platform improvement)
CREATE TABLE exercise_feedback (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    exercise_id UUID NOT NULL REFERENCES exercises(id) ON DELETE CASCADE,

    is_confusing BOOLEAN DEFAULT FALSE,
    has_errors BOOLEAN DEFAULT FALSE,
    difficulty_correct BOOLEAN DEFAULT FALSE,
    additional_notes TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_exercise_feedback_exercise_id ON exercise_feedback(exercise_id);

-- ============================================================================
-- AUDIT LOGGING (GDPR compliance)
-- ============================================================================

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,

    action VARCHAR(100) NOT NULL, -- 'user_login', 'exercise_attempt', 'data_export', etc.
    resource_type VARCHAR(50), -- 'exercise', 'skill', 'classroom'
    resource_id UUID,

    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);

-- ============================================================================
-- OFFLINE SYNC (CRDT-based)
-- ============================================================================

-- Client-side changes awaiting server sync
CREATE TABLE sync_queue (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    operation VARCHAR(10) NOT NULL, -- 'CREATE', 'UPDATE', 'DELETE'
    entity_type VARCHAR(50) NOT NULL, -- 'exercise_attempt', 'learner_skill_state'
    entity_id UUID NOT NULL,

    -- CRDT vector clock for ordering
    vector_clock JSONB NOT NULL, -- e.g., {"device_1": 5, "device_2": 3}

    payload JSONB NOT NULL, -- The actual change

    is_synced BOOLEAN DEFAULT FALSE,
    synced_at TIMESTAMP WITH TIME ZONE,

    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_sync_queue_user_id ON sync_queue(user_id);
CREATE INDEX idx_sync_queue_is_synced ON sync_queue(is_synced);

-- ============================================================================
-- CREATE UPDATED_AT TRIGGERS
-- ============================================================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_skills_updated_at BEFORE UPDATE ON skills
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_exercises_updated_at BEFORE UPDATE ON exercises
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_lessons_updated_at BEFORE UPDATE ON lessons
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_learner_skill_states_updated_at BEFORE UPDATE ON learner_skill_states
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_curriculum_paths_updated_at BEFORE UPDATE ON curriculum_paths
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_classrooms_updated_at BEFORE UPDATE ON classrooms
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_classroom_assignments_updated_at BEFORE UPDATE ON classroom_assignments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notification_preferences_updated_at BEFORE UPDATE ON notification_preferences
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- SEED DATA (Curriculum: Spanish A1)
-- ============================================================================

INSERT INTO curriculum_paths (code, name, description, language, is_published) VALUES
    ('spanish_a1', 'Spanish A1', 'Beginner Spanish - Common European Framework A1 Level', 'es', TRUE);

-- A1 Skills
INSERT INTO skills (code, name, description, level, language, category, estimated_time_minutes, is_published) VALUES
    ('spanish_a1_greetings', 'Greetings', 'Saying hello and goodbye', 'A1', 'es', 'vocabulary', 30, TRUE),
    ('spanish_a1_numbers', 'Numbers 1-20', 'Counting and basic numbers', 'A1', 'es', 'vocabulary', 45, TRUE),
    ('spanish_a1_colors', 'Colors', 'Basic color vocabulary', 'A1', 'es', 'vocabulary', 40, TRUE),
    ('spanish_a1_pronouns', 'Personal Pronouns', 'I, you, he, she, it, etc.', 'A1', 'es', 'grammar', 50, TRUE),
    ('spanish_a1_ser_estar', 'Ser vs Estar', 'To be: permanent vs temporary', 'A1', 'es', 'grammar', 60, TRUE);

-- Prerequisites
INSERT INTO skill_prerequisites (skill_id, prerequisite_skill_id, is_hard_prerequisite) VALUES
    ((SELECT id FROM skills WHERE code = 'spanish_a1_pronouns'), (SELECT id FROM skills WHERE code = 'spanish_a1_greetings'), TRUE),
    ((SELECT id FROM skills WHERE code = 'spanish_a1_ser_estar'), (SELECT id FROM skills WHERE code = 'spanish_a1_pronouns'), TRUE);

-- Curriculum ordering
INSERT INTO curriculum_path_skills (curriculum_path_id, skill_id, sequence, is_required) VALUES
    ((SELECT id FROM curriculum_paths WHERE code = 'spanish_a1'), (SELECT id FROM skills WHERE code = 'spanish_a1_greetings'), 1, TRUE),
    ((SELECT id FROM curriculum_paths WHERE code = 'spanish_a1'), (SELECT id FROM skills WHERE code = 'spanish_a1_numbers'), 2, TRUE),
    ((SELECT id FROM curriculum_paths WHERE code = 'spanish_a1'), (SELECT id FROM skills WHERE code = 'spanish_a1_colors'), 3, TRUE),
    ((SELECT id FROM curriculum_paths WHERE code = 'spanish_a1'), (SELECT id FROM skills WHERE code = 'spanish_a1_pronouns'), 4, TRUE),
    ((SELECT id FROM curriculum_paths WHERE code = 'spanish_a1'), (SELECT id FROM skills WHERE code = 'spanish_a1_ser_estar'), 5, TRUE);

-- Sample exercises for Greetings
INSERT INTO exercises (skill_id, type, title, prompt, correct_option, options, explanation) VALUES
    ((SELECT id FROM skills WHERE code = 'spanish_a1_greetings'),
     'multiple_choice',
     'Greeting in Spanish',
     'How do you say "Hello" in Spanish?',
     0,
     '["Hola", "Adiós", "Buenas noches", "¿Cómo estás?"]',
     'Hola is the most common greeting meaning "hello"'),
    ((SELECT id FROM skills WHERE code = 'spanish_a1_greetings'),
     'multiple_choice',
     'Saying goodbye',
     'Which phrase means "goodbye"?',
     1,
     '["Buenos días", "Adiós", "Gracias", "Por favor"]',
     'Adiós is the standard way to say goodbye in Spanish');

-- Sample lessons
INSERT INTO lessons (skill_id, sequence, title, description, learning_objectives) VALUES
    ((SELECT id FROM skills WHERE code = 'spanish_a1_greetings'), 1, 'Basic Greetings', 'Learn how to greet people in Spanish', '["Learn basic greetings", "Practice pronunciation", "Understand cultural nuances"]');

INSERT INTO lesson_exercises (lesson_id, exercise_id, sequence, is_required)
SELECT l.id, e.id, 1, TRUE
FROM lessons l
JOIN skills s ON l.skill_id = s.id
JOIN exercises e ON e.skill_id = s.id
WHERE s.code = 'spanish_a1_greetings'
LIMIT 1;

-- ============================================================================
-- STATISTICS
-- ============================================================================

/*
Total Tables: 30
Total Columns: 200+
Key Indexes: 80+

Storage estimate (1M learners):
- User data: ~500MB
- Exercise attempts: ~200GB (2TB of historical data)
- Skill states: ~100GB
- Total: ~300GB (with indexes)

Read patterns optimized for:
- User login (user_sessions)
- Exercise retrieval (exercises by skill)
- Spaced repetition scheduling (learner_skill_states with next_review_at)
- Progress tracking (learner_daily_metrics)
- Teacher dashboards (classroom enrollments, assignments)

Write patterns optimized for:
- Exercise attempt logging (exercise_attempts)
- Skill state updates (learner_skill_states)
- Offline sync queueing (sync_queue)

All timestamps use TIMESTAMP WITH TIME ZONE for timezone-aware operations.
All deletions are soft-deletes where appropriate (users.deleted_at).
*/
