# PATHFINDER Platform - Complete API Documentation

**Version**: 1.0.0  
**Status**: Production Ready  
**Last Updated**: 2026-06-11  

---

## Overview

PATHFINDER is a microservices-based learning platform with 9 independent services communicating via HTTP REST and gRPC. All services require authentication via `X-User-ID` header.

**Base URL**: `https://api.pathfinder.com/v1`  
**Authentication**: Bearer token or X-User-ID header  
**Response Format**: JSON  
**Rate Limit**: 100 requests/second per user  

---

## Service Directory

| Service | Port | Purpose | Status |
|---------|------|---------|--------|
| User Service | 8001 | Authentication, profiles | ✅ |
| Content Service | 8002 | Exercises, skills, curriculum | ✅ |
| Personalization Service | 8003 | BKT, HLR, adaptive difficulty | ✅ |
| Progress Service | 8004 | Exercise attempts, tracking | ✅ |
| Teacher Service | 8005 | Classroom management | ✅ |
| Parent Service | 8006 | Parent portal, child linking | ✅ |
| Notification Service | 8007 | Email, push, SMS delivery | ✅ |
| Achievement Service | 8008 | Badges, XP, leaderboards | ✅ |
| Insights Service | 8009 | Analytics, recommendations | ✅ |

---

## 1. USER SERVICE (Port 8001)

Manages user accounts, authentication, and profiles.

### Authentication

```
POST /v1/auth/register
Content-Type: application/json

{
  "email": "student@example.com",
  "password": "secure_password",
  "name": "John Doe",
  "role": "student",
  "dateOfBirth": "2010-05-15"
}

Response 201:
{
  "id": "user_123",
  "email": "student@example.com",
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "role": "student",
  "createdAt": "2026-06-11T10:00:00Z"
}
```

```
POST /v1/auth/login
Content-Type: application/json

{
  "email": "student@example.com",
  "password": "secure_password"
}

Response 200:
{
  "id": "user_123",
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "expiresIn": 86400
}
```

### Profile Management

```
GET /v1/users/:userId
Headers:
  X-User-ID: user_123

Response 200:
{
  "id": "user_123",
  "email": "student@example.com",
  "name": "John Doe",
  "role": "student",
  "dateOfBirth": "2010-05-15",
  "avatar": "https://cdn.pathfinder.com/avatars/user_123.jpg",
  "timezone": "America/New_York",
  "preferences": {
    "theme": "light",
    "language": "en"
  },
  "createdAt": "2026-06-11T10:00:00Z"
}
```

```
PUT /v1/users/:userId
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "name": "John Doe",
  "avatar": "avatar_url",
  "timezone": "America/New_York",
  "preferences": {
    "theme": "dark",
    "language": "en"
  }
}

Response 200: Updated user object
```

```
POST /v1/auth/verify-email
Content-Type: application/json

{
  "userId": "user_123",
  "code": "123456"
}

Response 200:
{
  "verified": true,
  "message": "Email verified successfully"
}
```

### Session Management

```
GET /v1/auth/session
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "email": "student@example.com",
  "active": true,
  "lastActivity": "2026-06-11T15:30:00Z",
  "expiresAt": "2026-06-12T10:00:00Z"
}
```

```
POST /v1/auth/logout
Headers:
  X-User-ID: user_123

Response 200:
{
  "message": "Logged out successfully"
}
```

### Data Export (GDPR)

```
POST /v1/users/:userId/export
Headers:
  X-User-ID: user_123

Response 202:
{
  "requestId": "export_req_123",
  "status": "processing",
  "estimatedTime": "24 hours"
}
```

```
GET /v1/users/:userId/export/:requestId
Headers:
  X-User-ID: user_123

Response 200:
{
  "requestId": "export_req_123",
  "status": "completed",
  "downloadUrl": "https://cdn.pathfinder.com/exports/export_req_123.zip",
  "expiresAt": "2026-06-18T15:30:00Z"
}
```

### Account Deletion (GDPR)

```
DELETE /v1/users/:userId
Headers:
  X-User-ID: user_123

Response 204: No Content
```

---

## 2. CONTENT SERVICE (Port 8002)

Manages exercises, skills, and curriculum.

### Skills

```
GET /v1/skills
Query Parameters:
  - grade: 1-12
  - subject: math, science, language
  - limit: 50
  - offset: 0

Response 200:
{
  "skills": [
    {
      "id": "skill_123",
      "name": "Fractions",
      "description": "Understanding basic fractions",
      "grade": 3,
      "subject": "math",
      "prerequisites": ["skill_120", "skill_121"],
      "exerciseCount": 42,
      "difficulty": "medium"
    }
  ],
  "total": 200,
  "limit": 50,
  "offset": 0
}
```

```
GET /v1/skills/:skillId
Headers:
  X-User-ID: user_123

Response 200:
{
  "id": "skill_123",
  "name": "Fractions",
  "description": "Understanding basic fractions",
  "grade": 3,
  "subject": "math",
  "prerequisites": ["skill_120", "skill_121"],
  "exercises": ["ex_100", "ex_101", "ex_102"],
  "learningResources": [
    {
      "id": "res_1",
      "title": "Introduction to Fractions",
      "type": "video",
      "url": "https://video.pathfinder.com/fractions_intro.mp4",
      "duration": 300
    }
  ]
}
```

### Exercises

```
GET /v1/exercises
Query Parameters:
  - skillId: skill_123
  - difficulty: easy|medium|hard
  - limit: 20
  - offset: 0

Response 200:
{
  "exercises": [
    {
      "id": "ex_100",
      "skillId": "skill_123",
      "question": "What is 1/2 + 1/4?",
      "difficulty": "easy",
      "type": "multiple_choice",
      "options": [
        "1/4",
        "3/4",
        "1/2",
        "1"
      ],
      "correctIndex": 1,
      "explanation": "Convert to same denominator: 2/4 + 1/4 = 3/4",
      "estimatedTime": 120
    }
  ],
  "total": 42,
  "limit": 20
}
```

```
GET /v1/exercises/:exerciseId
Headers:
  X-User-ID: user_123

Response 200:
{
  "id": "ex_100",
  "skillId": "skill_123",
  "question": "What is 1/2 + 1/4?",
  "difficulty": "easy",
  "type": "multiple_choice",
  "options": ["1/4", "3/4", "1/2", "1"],
  "correctIndex": 1,
  "explanation": "Convert to same denominator: 2/4 + 1/4 = 3/4",
  "relatedExercises": ["ex_101", "ex_102"],
  "hints": [
    "Find a common denominator",
    "2/4 and 1/4 have the same denominator"
  ]
}
```

### Curriculum Paths

```
GET /v1/curricula
Query Parameters:
  - grade: 3
  - subject: math

Response 200:
{
  "curricula": [
    {
      "id": "curr_1",
      "name": "Grade 3 Mathematics",
      "grade": 3,
      "skills": ["skill_120", "skill_121", "skill_122", "skill_123"],
      "duration": "9 months",
      "standards": ["Common Core 3.NF.A.1"]
    }
  ]
}
```

### Content Management (Teachers/Admins)

```
POST /v1/content/exercises (Admin only)
Headers:
  X-User-ID: user_admin
Content-Type: application/json

{
  "skillId": "skill_123",
  "question": "What is 1/2 + 1/4?",
  "difficulty": "easy",
  "type": "multiple_choice",
  "options": ["1/4", "3/4", "1/2", "1"],
  "correctIndex": 1,
  "explanation": "Convert to same denominator..."
}

Response 201:
{
  "id": "ex_200",
  "skillId": "skill_123",
  ...
}
```

---

## 3. PERSONALIZATION SERVICE (Port 8003)

Implements Bayesian Knowledge Tracing and Half-Life Regression.

### Knowledge State

```
GET /v1/personalization/knowledge/:userId/:skillId
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "skillId": "skill_123",
  "pKnow": 0.75,
  "confidence": 0.92,
  "lastUpdated": "2026-06-11T14:30:00Z",
  "bktParams": {
    "slip": 0.05,
    "guess": 0.15,
    "transit": 0.15
  },
  "learningCurve": [
    { "attempt": 1, "pKnow": 0.20 },
    { "attempt": 2, "pKnow": 0.35 },
    { "attempt": 3, "pKnow": 0.55 },
    { "attempt": 4, "pKnow": 0.75 }
  ]
}
```

### Difficulty Recommendation

```
POST /v1/personalization/recommend-difficulty
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "skillId": "skill_123"
}

Response 200:
{
  "recommendedDifficulty": "medium",
  "reasoning": "P(Know) = 0.75, optimal difficulty for learning",
  "alternativeDifficulties": ["easy", "hard"],
  "confidenceScore": 0.88
}
```

### Next Exercise Scheduling

```
POST /v1/personalization/schedule-next
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "skillId": "skill_123",
  "lastAttemptResult": true,
  "timeSinceLastAttempt": 86400
}

Response 200:
{
  "recommendedTime": "2026-06-12T14:00:00Z",
  "reason": "Based on Half-Life Regression: optimal retention",
  "priority": "medium",
  "halfLifeInDays": 3.2,
  "memoryStrength": 0.85
}
```

### Learning Model Calibration

```
GET /v1/personalization/calibrate/:userId
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "totalAttempts": 245,
  "skillsTracked": 18,
  "bktParametersLocked": false,
  "lastCalibration": "2026-06-10T08:00:00Z",
  "nextCalibrationDue": "2026-06-15T08:00:00Z"
}
```

---

## 4. PROGRESS SERVICE (Port 8004)

Tracks exercise attempts and overall progress.

### Submit Exercise Attempt

```
POST /v1/progress/attempts
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "exerciseId": "ex_100",
  "skillId": "skill_123",
  "selectedIndex": 1,
  "timeTaken": 45,
  "timestamp": "2026-06-11T15:30:00Z"
}

Response 201:
{
  "attemptId": "attempt_123",
  "exerciseId": "ex_100",
  "isCorrect": true,
  "timeTaken": 45,
  "masteryIncrease": 0.08,
  "newMastery": 0.75,
  "feedback": "Excellent! You understood the concept.",
  "nextExercise": "ex_101"
}
```

### Get Attempt History

```
GET /v1/progress/attempts/:userId
Query Parameters:
  - skillId: skill_123 (optional)
  - limit: 50
  - offset: 0

Response 200:
{
  "attempts": [
    {
      "attemptId": "attempt_123",
      "exerciseId": "ex_100",
      "skillId": "skill_123",
      "isCorrect": true,
      "timeTaken": 45,
      "timestamp": "2026-06-11T15:30:00Z"
    }
  ],
  "total": 245,
  "accuracy": 0.82
}
```

### Skill Progress

```
GET /v1/progress/skills/:userId
Query Parameters:
  - status: all|mastered|learning|struggling

Response 200:
{
  "skills": [
    {
      "skillId": "skill_123",
      "skillName": "Fractions",
      "masteryPercent": 75,
      "exercisesAttempted": 12,
      "exercisesCorrect": 10,
      "lastUpdated": "2026-06-11T15:30:00Z",
      "status": "learning",
      "trend": "up"
    }
  ],
  "overallMastery": 68
}
```

### Reset Progress (Teachers)

```
POST /v1/progress/reset/:userId/:skillId (Teacher only)
Headers:
  X-User-ID: teacher_123

Response 200:
{
  "message": "Progress reset",
  "skillId": "skill_123",
  "resetAt": "2026-06-11T15:45:00Z"
}
```

---

## 5. TEACHER SERVICE (Port 8005)

Manages classrooms, rosters, and teacher tools.

### Classroom Management

```
POST /v1/teachers/classrooms
Headers:
  X-User-ID: teacher_123
Content-Type: application/json

{
  "name": "Grade 3 Math - Morning",
  "grade": 3,
  "subject": "math",
  "maxStudents": 30,
  "inviteCode": "MATH3AM"
}

Response 201:
{
  "id": "classroom_456",
  "name": "Grade 3 Math - Morning",
  "grade": 3,
  "teacherId": "teacher_123",
  "inviteCode": "MATH3AM",
  "studentCount": 0,
  "createdAt": "2026-06-11T10:00:00Z"
}
```

```
GET /v1/teachers/classrooms/:classroomId
Headers:
  X-User-ID: teacher_123

Response 200:
{
  "id": "classroom_456",
  "name": "Grade 3 Math - Morning",
  "teacherId": "teacher_123",
  "students": ["user_123", "user_124", "user_125"],
  "skillPath": ["skill_120", "skill_121", "skill_122"],
  "studentCount": 3
}
```

### Roster Management

```
GET /v1/teachers/classrooms/:classroomId/roster
Headers:
  X-User-ID: teacher_123

Response 200:
{
  "students": [
    {
      "userId": "user_123",
      "name": "John Doe",
      "email": "john@example.com",
      "joinedAt": "2026-06-01T09:00:00Z",
      "status": "active"
    }
  ],
  "total": 3
}
```

```
POST /v1/teachers/classrooms/:classroomId/students
Headers:
  X-User-ID: teacher_123
Content-Type: application/json

{
  "email": "jane@example.com"
}

Response 201:
{
  "inviteSent": true,
  "inviteEmail": "jane@example.com"
}
```

### Real-Time Monitoring

```
GET /v1/teachers/classrooms/:classroomId/progress
Headers:
  X-User-ID: teacher_123

Response 200:
{
  "classProgress": {
    "averageMastery": 68,
    "studentsAboveTarget": 2,
    "studentsBelowTarget": 1,
    "skillsInProgress": ["skill_121", "skill_122"],
    "skillsCompleted": ["skill_120"]
  },
  "studentProgress": [
    {
      "userId": "user_123",
      "name": "John Doe",
      "mastery": 75,
      "status": "on-track",
      "flaggedSkills": []
    }
  ]
}
```

### Intervention Alerts

```
GET /v1/teachers/alerts/:classroomId
Headers:
  X-User-ID: teacher_123
Query Parameters:
  - severity: low|medium|high
  - resolved: true|false

Response 200:
{
  "alerts": [
    {
      "alertId": "alert_1",
      "studentId": "user_124",
      "studentName": "Jane Smith",
      "type": "struggling",
      "skillId": "skill_121",
      "skillName": "Decimals",
      "mastery": 25,
      "createdAt": "2026-06-11T14:00:00Z",
      "resolved": false
    }
  ],
  "total": 2
}
```

```
POST /v1/teachers/alerts/:alertId/intervene
Headers:
  X-User-ID: teacher_123
Content-Type: application/json

{
  "action": "assign_remedial",
  "skillId": "skill_121",
  "notes": "Review basic decimal concepts"
}

Response 200:
{
  "alertId": "alert_1",
  "resolved": true,
  "actionTaken": "assign_remedial"
}
```

---

## 6. PARENT SERVICE (Port 8006)

Parent portal for monitoring children's progress.

### Child Account Linking

```
POST /v1/parents/link-child
Headers:
  X-User-ID: parent_123
Content-Type: application/json

{
  "childEmail": "student@example.com"
}

Response 200:
{
  "linkRequested": true,
  "message": "Verification code sent to student's email"
}
```

```
POST /v1/parents/verify-link
Headers:
  X-User-ID: parent_123
Content-Type: application/json

{
  "childId": "user_123",
  "verificationCode": "123456"
}

Response 200:
{
  "linked": true,
  "childId": "user_123",
  "childName": "John Doe"
}
```

### View Linked Children

```
GET /v1/parents/children
Headers:
  X-User-ID: parent_123

Response 200:
{
  "children": [
    {
      "id": "user_123",
      "name": "John Doe",
      "grade": 3,
      "linkedAt": "2026-05-15T10:00:00Z",
      "lastActive": "2026-06-11T15:30:00Z"
    }
  ],
  "total": 1
}
```

### Child Progress

```
GET /v1/parents/children/:childId/progress
Headers:
  X-User-ID: parent_123

Response 200:
{
  "childId": "user_123",
  "childName": "John Doe",
  "overallMastery": 68,
  "skillsCompleted": 5,
  "totalSkills": 12,
  "streak": 7,
  "lastActivityAt": "2026-06-11T15:30:00Z"
}
```

### Learning Curve

```
GET /v1/parents/children/:childId/learning-curve
Headers:
  X-User-ID: parent_123
Query Parameters:
  - timeRange: week|month|all

Response 200:
{
  "childId": "user_123",
  "timeRange": "month",
  "data": [
    {
      "date": "2026-06-01",
      "averageMastery": 60,
      "exercisesCompleted": 5,
      "accuracy": 0.80
    }
  ],
  "trend": "improving"
}
```

### Skills Overview

```
GET /v1/parents/children/:childId/skills
Headers:
  X-User-ID: parent_123

Response 200:
{
  "skills": [
    {
      "skillId": "skill_120",
      "skillName": "Place Value",
      "mastery": 85,
      "status": "mastered",
      "completedAt": "2026-06-05T14:00:00Z"
    },
    {
      "skillId": "skill_121",
      "skillName": "Decimals",
      "mastery": 25,
      "status": "struggling",
      "lastAttempt": "2026-06-11T15:30:00Z"
    }
  ]
}
```

### Recommendations for Parents

```
GET /v1/parents/children/:childId/recommendations
Headers:
  X-User-ID: parent_123

Response 200:
{
  "recommendations": [
    {
      "id": "rec_1",
      "type": "support_needed",
      "skillId": "skill_121",
      "skillName": "Decimals",
      "reason": "Mastery only 25%, student struggling",
      "suggestedAction": "Review fundamental concepts with student",
      "priority": "high"
    }
  ]
}
```

---

## 7. NOTIFICATION SERVICE (Port 8007)

Manages email, push, and SMS notifications.

### Update Preferences

```
POST /v1/notifications/preferences
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "emailNotifications": {
    "masteryMilestone": true,
    "alert": true,
    "dailySummary": false,
    "weeklyReport": true,
    "achievements": true
  },
  "pushNotifications": {
    "enabled": true,
    "masteryMilestone": true,
    "alerts": true
  },
  "smsNotifications": {
    "enabled": false
  },
  "emailFrequency": "daily",
  "quietHours": {
    "enabled": true,
    "startTime": "21:00",
    "endTime": "08:00",
    "timezone": "America/New_York"
  }
}

Response 200:
{
  "message": "Preferences updated"
}
```

### Get Preferences

```
GET /v1/notifications/preferences
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "emailNotifications": { ... },
  "pushNotifications": { ... },
  "emailFrequency": "daily",
  "quietHours": { ... }
}
```

### Send Notification (Admin)

```
POST /v1/notifications/send (Admin only)
Headers:
  X-User-ID: admin_123
Content-Type: application/json

{
  "userId": "user_123",
  "type": "achievement",
  "title": "Congratulations!",
  "message": "You've mastered fractions!",
  "channels": ["email", "push"],
  "data": {
    "badgeId": "badge_456",
    "skillId": "skill_123"
  }
}

Response 200:
{
  "notificationId": "notif_789",
  "sent": true,
  "channels": {
    "email": "queued",
    "push": "sent"
  }
}
```

### Notification History

```
GET /v1/notifications
Headers:
  X-User-ID: user_123
Query Parameters:
  - limit: 20
  - offset: 0
  - read: true|false|all

Response 200:
{
  "notifications": [
    {
      "id": "notif_789",
      "type": "achievement",
      "title": "Congratulations!",
      "message": "You've mastered fractions!",
      "read": false,
      "createdAt": "2026-06-11T15:30:00Z",
      "channels": ["email", "push"]
    }
  ],
  "total": 15,
  "unreadCount": 3
}
```

### Mark as Read

```
PUT /v1/notifications/:notificationId
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "read": true
}

Response 200:
{
  "id": "notif_789",
  "read": true
}
```

---

## 8. ACHIEVEMENT SERVICE (Port 8008)

Badges, XP, gamification, and leaderboards.

### Achievements

```
GET /v1/achievements
Headers:
  X-User-ID: user_123
Query Parameters:
  - unlocked: true|false|all
  - category: skill|streak|speed
  - limit: 50

Response 200:
{
  "achievements": [
    {
      "id": "badge_100",
      "name": "First Steps",
      "description": "Complete your first exercise",
      "category": "milestone",
      "rarity": "common",
      "points": 10,
      "unlocked": true,
      "unlockedAt": "2026-06-01T09:00:00Z",
      "icon": "https://cdn.pathfinder.com/badges/first_steps.png"
    }
  ],
  "total": 42,
  "unlockedCount": 15
}
```

### Unlock Achievement

```
POST /v1/achievements/unlock (Internal use)
Headers:
  X-User-ID: system
Content-Type: application/json

{
  "userId": "user_123",
  "badgeId": "badge_100",
  "timestamp": "2026-06-11T15:30:00Z"
}

Response 201:
{
  "userId": "user_123",
  "badgeId": "badge_100",
  "unlockedAt": "2026-06-11T15:30:00Z",
  "pointsAwarded": 10,
  "totalPoints": 150
}
```

### User Stats

```
GET /v1/achievements/stats/:userId
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "totalPoints": 150,
  "level": 3,
  "xpToNextLevel": 45,
  "achievementCount": 15,
  "totalBadges": 42,
  "rank": 127,
  "percentile": 82,
  "streak": 7
}
```

### Goals

```
POST /v1/goals
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "type": "skills_to_master",
  "skillId": "skill_123",
  "difficulty": "hard",
  "deadline": "2026-07-11T23:59:59Z",
  "durationMinutes": 45
}

Response 201:
{
  "id": "goal_123",
  "userId": "user_123",
  "type": "skills_to_master",
  "skillId": "skill_123",
  "deadline": "2026-07-11T23:59:59Z",
  "status": "active",
  "createdAt": "2026-06-11T15:30:00Z"
}
```

```
GET /v1/goals
Headers:
  X-User-ID: user_123
Query Parameters:
  - status: active|completed|expired

Response 200:
{
  "goals": [
    {
      "id": "goal_123",
      "type": "skills_to_master",
      "skillId": "skill_123",
      "deadline": "2026-07-11T23:59:59Z",
      "status": "active",
      "progress": 0.65
    }
  ],
  "total": 3,
  "activeCount": 2
}
```

### Leaderboard

```
GET /v1/leaderboard
Query Parameters:
  - timeRange: week|month|all
  - limit: 100
  - offset: 0

Response 200:
{
  "leaderboard": [
    {
      "rank": 1,
      "userId": "user_456",
      "name": "Alex Chen",
      "points": 5200,
      "achievements": 48,
      "mastery": 95,
      "streak": 45
    }
  ],
  "total": 1000,
  "userRank": 127,
  "userPercentile": 82
}
```

### User Rank

```
GET /v1/leaderboard/rank/:userId
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "name": "John Doe",
  "rank": 127,
  "points": 150,
  "percentile": 82,
  "rankedUsersAbove": 126,
  "rankedUsersBelow": 874
}
```

---

## 9. INSIGHTS SERVICE (Port 8009)

Analytics, recommendations, and study planning.

### Analytics

```
GET /v1/insights/analytics
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "totalSkills": 12,
  "skillsMastered": 5,
  "skillsLearning": 5,
  "skillsStruggling": 2,
  "averageMastery": 68,
  "averageAccuracy": 0.82,
  "totalTimeSpent": 3600,
  "currentStreak": 7,
  "longestStreak": 15,
  "learningTrend": "improving"
}
```

### Recommendations

```
GET /v1/insights/recommendations
Headers:
  X-User-ID: user_123
Query Parameters:
  - limit: 10

Response 200:
{
  "recommendations": [
    {
      "id": "rec_1",
      "type": "practice",
      "skillId": "skill_122",
      "skillName": "Division",
      "reason": "Close to mastery (0.82), practice more to reach 85%",
      "priority": "medium",
      "timeNeeded": 45,
      "exerciseCount": 5
    },
    {
      "id": "rec_2",
      "type": "review",
      "skillId": "skill_120",
      "skillName": "Place Value",
      "reason": "Last attempted 5 days ago, optimal time for review",
      "priority": "high",
      "timeNeeded": 30,
      "exerciseCount": 3
    }
  ]
}
```

### Generate Recommendations (Force Update)

```
POST /v1/insights/recommendations/generate
Headers:
  X-User-ID: user_123

Response 200:
{
  "generated": true,
  "count": 8,
  "generatedAt": "2026-06-11T15:45:00Z"
}
```

### Study Plans

```
POST /v1/insights/study-plan
Headers:
  X-User-ID: user_123
Content-Type: application/json

{
  "duration": 45,
  "skillFocus": "skill_122",
  "difficulty": "medium",
  "scheduledTime": "2026-06-11T17:00:00Z"
}

Response 201:
{
  "planId": "plan_123",
  "duration": 45,
  "skillFocus": "skill_122",
  "scheduledTime": "2026-06-11T17:00:00Z",
  "exercises": ["ex_100", "ex_101", "ex_102"],
  "status": "scheduled"
}
```

```
GET /v1/insights/study-plan
Headers:
  X-User-ID: user_123
Query Parameters:
  - status: scheduled|in-progress|completed

Response 200:
{
  "plans": [
    {
      "planId": "plan_123",
      "duration": 45,
      "skillFocus": "skill_122",
      "status": "scheduled",
      "scheduledTime": "2026-06-11T17:00:00Z",
      "estimatedCompletion": "2026-06-11T17:45:00Z"
    }
  ],
  "total": 3,
  "upcomingCount": 1
}
```

### Learning Style

```
GET /v1/insights/learning-style
Headers:
  X-User-ID: user_123

Response 200:
{
  "userId": "user_123",
  "dominantStyle": "visual",
  "styles": {
    "visual": 0.45,
    "auditory": 0.25,
    "kinesthetic": 0.20,
    "reading": 0.10
  },
  "recommendations": [
    "Watch video tutorials for better understanding",
    "Use diagrams and visual aids",
    "Try interactive simulations"
  ]
}
```

### Performance Metrics

```
GET /v1/insights/performance
Headers:
  X-User-ID: user_123
Query Parameters:
  - timeRange: week|month|all

Response 200:
{
  "userId": "user_123",
  "timeRange": "month",
  "totalExercises": 45,
  "correctAnswers": 37,
  "accuracy": 0.82,
  "averageTimeTaken": 54,
  "skillsImproved": 3,
  "skillsRegressed": 1,
  "totalTimeSpent": 3600,
  "dailyAverage": 120,
  "weeklyProgress": [
    {
      "week": "2026-06-01",
      "exercises": 12,
      "accuracy": 0.80,
      "timeSpent": 900
    }
  ]
}
```

---

## Error Handling

All services return standardized error responses:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Email already exists",
    "details": {
      "field": "email"
    },
    "timestamp": "2026-06-11T15:30:00Z"
  }
}
```

### Common Error Codes

| Code | Status | Meaning |
|------|--------|---------|
| `INVALID_REQUEST` | 400 | Invalid request parameters |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `CONFLICT` | 409 | Resource already exists |
| `UNPROCESSABLE_ENTITY` | 422 | Validation failed |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Rate Limiting

- **Default**: 100 requests/second per user
- **Burst**: 200 requests in 10-second window
- **Headers**:
  ```
  X-RateLimit-Limit: 100
  X-RateLimit-Remaining: 87
  X-RateLimit-Reset: 1623428400
  ```

---

## Pagination

All list endpoints support pagination:

```
Query Parameters:
  - limit: 1-100 (default: 50)
  - offset: 0+ (default: 0)

Response:
{
  "items": [...],
  "total": 500,
  "limit": 50,
  "offset": 0,
  "hasNext": true,
  "hasPrev": false
}
```

---

## Webhooks

Services support webhook callbacks for events:

```json
POST https://your-domain.com/webhooks/pathfinder

{
  "eventId": "evt_123",
  "type": "achievement.unlocked",
  "timestamp": "2026-06-11T15:30:00Z",
  "data": {
    "userId": "user_123",
    "badgeId": "badge_100",
    "points": 10
  }
}
```

Register webhook: `POST /v1/webhooks` (Admin only)

---

## Status Codes

| Code | Usage |
|------|-------|
| 200 | OK |
| 201 | Created |
| 202 | Accepted (async) |
| 204 | No content |
| 400 | Bad request |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not found |
| 422 | Validation error |
| 429 | Rate limited |
| 500 | Server error |
| 503 | Service unavailable |

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0.0 | 2026-06-11 | Initial release |

---

**For more information**: https://docs.pathfinder.com  
**Support**: support@pathfinder.com  
**API Status**: https://status.pathfinder.com
