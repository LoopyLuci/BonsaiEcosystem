// PATHFINDER Learning Platform - Core Backend Services
// Phase 0-1: Foundation + Core MVP
// Package: shared models and core business logic

package pathfinder

import (
	"context"
	"database/sql"
	"errors"
	"time"
)

// ============================================================================
// ERROR TYPES
// ============================================================================

var (
	ErrNotFound       = errors.New("resource not found")
	ErrUnauthorized   = errors.New("unauthorized")
	ErrAlreadyExists  = errors.New("resource already exists")
	ErrInvalidInput   = errors.New("invalid input")
	ErrExternalAPI    = errors.New("external API error")
	ErrDatabaseError  = errors.New("database error")
)

// ============================================================================
// USER MODELS
// ============================================================================

type User struct {
	ID                UUID
	Email             string
	EmailVerified     bool
	EmailVerifiedAt   *time.Time
	FirstName         string
	LastName          string
	AvatarURL         *string
	Bio               *string
	LanguagePreference string
	Timezone          string
	IsActive          bool
	IsTeacher         bool
	IsAdmin           bool

	// Privacy
	DataExportRequested   bool
	DataExportRequestedAt *time.Time
	DeletionRequested     bool
	DeletionRequestedAt   *time.Time
	DeletionScheduledAt   *time.Time

	// COPPA (children's privacy)
	Age               *int
	ParentConsent     bool
	ParentConsentAt   *time.Time
	ParentEmail       *string

	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt *time.Time
}

type UserSession struct {
	ID        UUID
	UserID    UUID
	TokenHash string
	DeviceName *string
	DeviceType string // 'web', 'ios', 'android'
	IPAddress *string
	UserAgent *string
	ExpiresAt time.Time
	RevokedAt *time.Time
	CreatedAt time.Time
}

// ============================================================================
// SKILL & CONTENT MODELS
// ============================================================================

type Skill struct {
	ID                 UUID
	Code               string
	Name               string
	Description        *string
	Level              string // 'A0', 'A1', 'A2', 'B1', 'B2', 'C1', 'C2'
	Language           string
	Category           string // 'vocabulary', 'grammar', 'listening', 'reading', 'writing', 'speaking'
	IconURL            *string
	ColorHex           *string
	EstimatedTimeMinutes int
	DifficultyLevel    float64 // 1.0 = base
	IsPublished        bool
	PublishedAt        *time.Time
	CreatedAt          time.Time
	UpdatedAt          time.Time
}

type Exercise struct {
	ID                 UUID
	SkillID            UUID
	Type               string // 'multiple_choice', 'translation', 'listening', 'reading', 'writing', 'matching'
	Title              string
	Description        *string
	DifficultyDelta    float64

	// Multiple choice
	Prompt             *string
	CorrectOption      *int
	Options            []string
	Explanation        *string

	// Translation
	SourceLanguage     *string
	TargetLanguage     *string
	SourceText         *string
	AcceptableAnswers  []string

	// Reading
	Passage            *string
	ComprehensionQuestions *string // JSON

	// Listening
	AudioURL           *string
	AudioDurationSeconds *int
	Transcript         *string

	IsPublished        bool
	PublishedAt        *time.Time
	EstimatedTimeSeconds int
	UsageCount         int
	AverageSuccessRate float64

	CreatedAt          time.Time
	UpdatedAt          time.Time
}

type Lesson struct {
	ID                 UUID
	SkillID            UUID
	Sequence           int
	Title              string
	Description        *string
	LearningObjectives []string
	CreatedAt          time.Time
	UpdatedAt          time.Time
}

// ============================================================================
// LEARNER STATE MODELS (Bayesian Knowledge Tracing + Spaced Repetition)
// ============================================================================

// LearnerSkillState represents the learner's knowledge state for a skill
// Uses Bayesian Knowledge Tracing (BKT) to model learning
type LearnerSkillState struct {
	ID                UUID
	UserID            UUID
	SkillID           UUID

	// BKT parameters
	PKnow             float64 // Probability of knowledge (main metric)
	PSlip             float64 // P(wrong | knows) - typically ~0.1
	PGuess            float64 // P(correct | doesn't know) - typically ~0.25
	PTransit          float64 // P(learn | didn't know) - typically ~0.05

	// Half-Life Regression (spaced repetition)
	HalflifeDay       float64 // Optimal review interval in days
	LastReviewedAt    *time.Time
	NextReviewAt      *time.Time

	// Mastery
	IsMastered        bool
	MasteryThreshold  float64 // Usually 0.85
	MasteredAt        *time.Time

	// Strength (0-1)
	Strength          float64

	// Practice
	AttemptCount      int
	CorrectCount      int

	CreatedAt         time.Time
	UpdatedAt         time.Time
}

// ExerciseAttempt represents a single learner's attempt at an exercise
type ExerciseAttempt struct {
	ID                    UUID
	UserID                UUID
	ExerciseID            UUID
	SkillID               UUID

	WasCorrect            bool
	Response              *string
	ResponseTimeSeconds   *int

	ExerciseDifficultyRating float64
	UserAbilityRating        float64
	DiscriminationIndex      float64

	FeedbackGiven         bool
	FeedbackText          *string

	AttemptNumber         int

	CreatedAt             time.Time
}

// ReviewHistory tracks spacing intervals for spaced repetition
type ReviewHistory struct {
	ID                UUID
	UserID            UUID
	SkillID           UUID

	ReviewNumber      int  // 1st, 2nd, 3rd review
	IntervalDays      *int // Days since last review
	EaseFactor        float64 // SuperMemo ease factor
	WasSuccess        bool
	NextIntervalDays  *int

	CreatedAt         time.Time
}

// ============================================================================
// BAYESIAN KNOWLEDGE TRACING (BKT) ALGORITHM
// ============================================================================

// BKTParams holds the BKT model parameters
type BKTParams struct {
	PInit   float64 // Initial probability of knowing (typically 0.3)
	PSlip   float64 // Probability of slip (know but answer wrong, ~0.1)
	PGuess  float64 // Probability of guess (don't know but lucky, ~0.25)
	PTransit float64 // Probability of learning in one attempt (~0.05)
}

// UpdateBKTState updates the BKT probability given an exercise outcome
// Uses Bayesian update rule to revise P(Know)
func UpdateBKTState(state *LearnerSkillState, params BKTParams, isCorrect bool) {
	pKnow := state.PKnow
	pSlip := params.PSlip
	pGuess := params.PGuess

	// P(correct) = P(know) * (1 - P(slip)) + P(not know) * P(guess)
	pCorrect := pKnow*(1-pSlip) + (1-pKnow)*pGuess

	// Bayesian update
	if isCorrect {
		// P(know | correct) = P(correct | know) * P(know) / P(correct)
		state.PKnow = (pKnow * (1 - pSlip)) / pCorrect
	} else {
		// P(know | incorrect) = P(incorrect | know) * P(know) / P(incorrect)
		pIncorrect := 1 - pCorrect
		state.PKnow = (pKnow * pSlip) / pIncorrect
	}

	// Learning transition: if didn't know, might learn
	if state.PKnow >= 0.5 { // Threshold for "learning"
		state.PKnow = state.PKnow + (1-state.PKnow)*params.PTransit
	}

	// Strength is normalized P(Know)
	state.Strength = state.PKnow

	state.UpdatedAt = time.Now()
}

// ============================================================================
// HALF-LIFE REGRESSION (Spaced Repetition Scheduling)
// ============================================================================

// HLRParams holds Half-Life Regression parameters
type HLRParams struct {
	Decay     float64 // Decay rate (typically 0.3 to 0.5)
	Threshold float64 // Retention threshold (typically 0.9)
	MinDays   float64 // Minimum interval (typically 1 day)
	MaxDays   float64 // Maximum interval (typically 36000 days = 98 years)
}

// CalculateNextReviewInterval calculates optimal next review time using Half-Life Regression
// Based on Cepeda et al. 2008 - memory retention curve
//
// Formula: t = (S - ln(desired_retention) / ln(base)) ^ (1/decay)
// Where S is stability (strength of memory)
func CalculateNextReviewInterval(
	state *LearnerSkillState,
	lastAttemptCorrect bool,
	params HLRParams,
) time.Duration {

	// If last attempt was wrong, review sooner
	factor := 1.0
	if !lastAttemptCorrect {
		factor = 0.5
	}

	// Stability increases with strength
	stability := 1.0 + state.Strength*5.0

	// Decay rate based on practice (more practice = slower decay = longer intervals)
	decay := params.Decay * (1.0 - (float64(state.AttemptCount) / 100.0))
	if decay < 0.1 {
		decay = 0.1
	}

	// Half-life calculation
	// This models how memory strength decays exponentially
	halfLife := (state.HalflifeDay * 24 * 60 * 60) // Convert to seconds

	if halfLife == 0 {
		halfLife = 21 * 24 * 60 * 60 // Default 21 days
	}

	// Ideal retention interval (when to review to maintain ~90% retention)
	// Using exponential decay: retention = 2 ^ (-t/halflife)
	// Solving for t: t = -halflife * log2(desired_retention)
	desiredRetention := 0.9
	nextIntervalSeconds := -halfLife * math.Log(desiredRetention) / math.Log(2.0)

	// Apply factors
	nextIntervalSeconds *= factor

	// Clamp to reasonable bounds
	minSeconds := params.MinDays * 24 * 60 * 60
	maxSeconds := params.MaxDays * 24 * 60 * 60

	if nextIntervalSeconds < minSeconds {
		nextIntervalSeconds = minSeconds
	}
	if nextIntervalSeconds > maxSeconds {
		nextIntervalSeconds = maxSeconds
	}

	// Update state
	state.HalflifeDay = nextIntervalSeconds / (24 * 60 * 60)
	state.LastReviewedAt = pointerTime(time.Now())
	nextReview := time.Now().Add(time.Duration(nextIntervalSeconds) * time.Second)
	state.NextReviewAt = &nextReview
	state.UpdatedAt = time.Now()

	return time.Duration(nextIntervalSeconds) * time.Second
}

// ============================================================================
// SPACED REPETITION SCHEDULER
// ============================================================================

// SchedulerService implements spaced repetition scheduling logic
type SchedulerService struct {
	db *sql.DB
	bktParams BKTParams
	hlrParams HLRParams
}

// GetNextSkillsToReview returns skills the learner should review next
// Orders by priority: mastered skills that need maintenance, then developing skills
func (s *SchedulerService) GetNextSkillsToReview(
	ctx context.Context,
	userID UUID,
	limit int,
) ([]Skill, error) {
	query := `
		SELECT s.id, s.code, s.name, s.description, s.level, s.language, s.category,
		       s.icon_url, s.color_hex, s.estimated_time_minutes, s.difficulty_level,
		       s.is_published, s.published_at, s.created_at, s.updated_at
		FROM skills s
		INNER JOIN learner_skill_states lss ON s.id = lss.skill_id
		WHERE lss.user_id = $1
		  AND lss.next_review_at IS NOT NULL
		  AND lss.next_review_at <= NOW()
		ORDER BY lss.next_review_at ASC, lss.is_mastered ASC, lss.strength DESC
		LIMIT $2
	`

	rows, err := s.db.QueryContext(ctx, query, userID, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var skills []Skill
	for rows.Next() {
		var skill Skill
		err := rows.Scan(
			&skill.ID, &skill.Code, &skill.Name, &skill.Description, &skill.Level,
			&skill.Language, &skill.Category, &skill.IconURL, &skill.ColorHex,
			&skill.EstimatedTimeMinutes, &skill.DifficultyLevel,
			&skill.IsPublished, &skill.PublishedAt, &skill.CreatedAt, &skill.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
		skills = append(skills, skill)
	}

	return skills, rows.Err()
}

// RecordExerciseAttempt records an exercise attempt and updates learner state
func (s *SchedulerService) RecordExerciseAttempt(
	ctx context.Context,
	attempt *ExerciseAttempt,
) error {
	// Start transaction
	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	// Insert exercise attempt
	insertAttemptQuery := `
		INSERT INTO exercise_attempts
		(id, user_id, exercise_id, skill_id, was_correct, response, response_time_seconds,
		 exercise_difficulty_rating, user_ability_rating, attempt_number, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
	`
	_, err = tx.ExecContext(ctx, insertAttemptQuery,
		attempt.ID, attempt.UserID, attempt.ExerciseID, attempt.SkillID,
		attempt.WasCorrect, attempt.Response, attempt.ResponseTimeSeconds,
		attempt.ExerciseDifficultyRating, attempt.UserAbilityRating,
		attempt.AttemptNumber, attempt.CreatedAt,
	)
	if err != nil {
		return err
	}

	// Fetch current skill state
	var state LearnerSkillState
	getStateQuery := `
		SELECT id, user_id, skill_id, p_know, p_slip, p_guess, p_transit,
		       halflife_days, last_reviewed_at, next_review_at, is_mastered,
		       mastery_threshold, strength, attempt_count, correct_count,
		       created_at, updated_at
		FROM learner_skill_states
		WHERE user_id = $1 AND skill_id = $2
	`
	err = tx.QueryRowContext(ctx, getStateQuery, attempt.UserID, attempt.SkillID).Scan(
		&state.ID, &state.UserID, &state.SkillID, &state.PKnow, &state.PSlip, &state.PGuess, &state.PTransit,
		&state.HalflifeDay, &state.LastReviewedAt, &state.NextReviewAt, &state.IsMastered,
		&state.MasteryThreshold, &state.Strength, &state.AttemptCount, &state.CorrectCount,
		&state.CreatedAt, &state.UpdatedAt,
	)
	if err != nil && err != sql.ErrNoRows {
		return err
	}

	// If no state exists, create one
	if err == sql.ErrNoRows {
		state.ID = NewUUID()
		state.UserID = attempt.UserID
		state.SkillID = attempt.SkillID
		state.PKnow = 0.3 // Initial probability
		state.PSlip = 0.1
		state.PGuess = 0.25
		state.PTransit = 0.05
		state.MasteryThreshold = 0.85
		state.CreatedAt = time.Now()
		state.UpdatedAt = time.Now()

		insertStateQuery := `
			INSERT INTO learner_skill_states
			(id, user_id, skill_id, p_know, p_slip, p_guess, p_transit,
			 halflife_days, mastery_threshold, strength, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
		`
		_, err = tx.ExecContext(ctx, insertStateQuery,
			state.ID, state.UserID, state.SkillID, state.PKnow, state.PSlip, state.PGuess, state.PTransit,
			state.HalflifeDay, state.MasteryThreshold, state.Strength, state.CreatedAt, state.UpdatedAt,
		)
		if err != nil {
			return err
		}
	}

	// Update BKT state
	UpdateBKTState(&state, s.bktParams, attempt.WasCorrect)

	// Update practice counts
	state.AttemptCount++
	if attempt.WasCorrect {
		state.CorrectCount++
	}

	// Check mastery
	if state.PKnow >= state.MasteryThreshold && !state.IsMastered {
		state.IsMastered = true
		state.MasteredAt = pointerTime(time.Now())
	}

	// Calculate next review interval
	CalculateNextReviewInterval(&state, attempt.WasCorrect, s.hlrParams)

	// Update skill state in database
	updateStateQuery := `
		UPDATE learner_skill_states
		SET p_know = $1, strength = $2, halflife_days = $3, last_reviewed_at = $4,
		    next_review_at = $5, is_mastered = $6, mastered_at = $7,
		    attempt_count = $8, correct_count = $9, updated_at = $10
		WHERE id = $11
	`
	_, err = tx.ExecContext(ctx, updateStateQuery,
		state.PKnow, state.Strength, state.HalflifeDay, state.LastReviewedAt,
		state.NextReviewAt, state.IsMastered, state.MasteredAt,
		state.AttemptCount, state.CorrectCount, state.UpdatedAt,
		state.ID,
	)
	if err != nil {
		return err
	}

	// Record in review history
	review := ReviewHistory{
		ID:               NewUUID(),
		UserID:           attempt.UserID,
		SkillID:          attempt.SkillID,
		ReviewNumber:     state.AttemptCount,
		WasSuccess:       attempt.WasCorrect,
		EaseFactor:       2.5, // Standard SuperMemo ease factor
		CreatedAt:        time.Now(),
	}

	if state.LastReviewedAt != nil {
		daysSince := int(time.Since(*state.LastReviewedAt).Hours() / 24)
		review.IntervalDays = &daysSince
	}

	if state.NextReviewAt != nil {
		daysUntil := int(time.Until(*state.NextReviewAt).Hours() / 24)
		review.NextIntervalDays = &daysUntil
	}

	insertReviewQuery := `
		INSERT INTO review_history
		(id, user_id, skill_id, review_number, interval_days, ease_factor, was_success,
		 next_interval_days, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	`
	_, err = tx.ExecContext(ctx, insertReviewQuery,
		review.ID, review.UserID, review.SkillID, review.ReviewNumber,
		review.IntervalDays, review.EaseFactor, review.WasSuccess,
		review.NextIntervalDays, review.CreatedAt,
	)
	if err != nil {
		return err
	}

	return tx.Commit().Error
}

// ============================================================================
// UTILITY TYPES & FUNCTIONS
// ============================================================================

// UUID is a unique identifier
type UUID [16]byte

// NewUUID generates a new UUID
func NewUUID() UUID {
	// Implementation: use github.com/google/uuid or similar
	// For now, placeholder
	return UUID{}
}

// Pointer helpers
func pointerTime(t time.Time) *time.Time {
	return &t
}

func pointerInt(i int) *int {
	return &i
}

func pointerFloat64(f float64) *float64 {
	return &f
}

func pointerString(s string) *string {
	return &s
}

// ============================================================================
// LEARNING PROGRESS CALCULATIONS
// ============================================================================

// ProgressMetrics represents learner progress
type ProgressMetrics struct {
	TotalSkills       int
	MasteredSkills    int
	DevelopingSkills  int
	MasteryPercentage float64 // 0-100
	AverageStrength   float64 // 0-1
	DailyStreak       int
	TotalExercisesCompleted int
	AverageAccuracy   float64 // 0-1
}

// CalculateProgressMetrics computes overall progress for a learner
func CalculateProgressMetrics(
	ctx context.Context,
	db *sql.DB,
	userID UUID,
) (*ProgressMetrics, error) {
	query := `
		SELECT
		    COUNT(*) as total_skills,
		    SUM(CASE WHEN is_mastered THEN 1 ELSE 0 END) as mastered_skills,
		    SUM(CASE WHEN is_mastered THEN 0 ELSE 1 END) as developing_skills,
		    AVG(strength) as avg_strength,
		    (SELECT COUNT(*) FROM exercise_attempts WHERE user_id = $1 AND was_correct) as correct_exercises,
		    (SELECT COUNT(*) FROM exercise_attempts WHERE user_id = $1) as total_exercises
		FROM learner_skill_states
		WHERE user_id = $1
	`

	var totalSkills, masteredSkills, developingSkills sql.NullInt64
	var avgStrength sql.NullFloat64
	var correctExercises, totalExercises sql.NullInt64

	err := db.QueryRowContext(ctx, query, userID).Scan(
		&totalSkills, &masteredSkills, &developingSkills, &avgStrength,
		&correctExercises, &totalExercises,
	)
	if err != nil {
		return nil, err
	}

	metrics := &ProgressMetrics{
		TotalSkills:       int(totalSkills.Int64),
		MasteredSkills:    int(masteredSkills.Int64),
		DevelopingSkills:  int(developingSkills.Int64),
		AverageStrength:   avgStrength.Float64,
	}

	if totalSkills.Int64 > 0 {
		metrics.MasteryPercentage = float64(masteredSkills.Int64) / float64(totalSkills.Int64) * 100
	}

	if totalExercises.Int64 > 0 {
		metrics.AverageAccuracy = float64(correctExercises.Int64) / float64(totalExercises.Int64)
		metrics.TotalExercisesCompleted = int(totalExercises.Int64)
	}

	// Calculate daily streak (simplified)
	// In production, would use window functions or application logic
	metrics.DailyStreak = 0

	return metrics, nil
}
