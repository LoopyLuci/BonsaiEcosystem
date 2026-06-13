# PATHFINDER Week 5 - Teacher Dashboard
## Classroom Management, Student Monitoring, Real-Time Analytics (8,000 LOC)

**Timeline**: Week 16-20 (2026-06-16 to 2026-06-20)  
**Target**: 8,000 LOC of new backend + frontend code  
**Status**: 🚀 READY TO BUILD  

---

## 📋 WEEK 5 ARCHITECTURE

### Backend Services (2,000 LOC new)
**Teacher Service** (new microservice on port 8005)
```go
// Endpoints:
POST   /v1/teachers/classrooms          // Create classroom
GET    /v1/teachers/classrooms          // List teacher's classrooms
GET    /v1/teachers/classrooms/:id      // Get classroom details
PUT    /v1/teachers/classrooms/:id      // Update classroom
DELETE /v1/teachers/classrooms/:id      // Delete classroom

POST   /v1/teachers/classrooms/:id/students          // Add student
GET    /v1/teachers/classrooms/:id/students          // List students
DELETE /v1/teachers/classrooms/:id/students/:uid     // Remove student

GET    /v1/teachers/classrooms/:id/progress          // Class progress overview
GET    /v1/teachers/classrooms/:id/skills/:sid       // Per-skill class stats
GET    /v1/teachers/classrooms/:id/alerts            // Intervention alerts
POST   /v1/teachers/classrooms/:id/reports/export    // Export class report
```

**Database** (3 new tables - 300 lines SQL)
```sql
CREATE TABLE classrooms (
  id UUID PRIMARY KEY,
  teacher_id UUID NOT NULL REFERENCES users(id),
  name VARCHAR(255) NOT NULL,
  description TEXT,
  subject VARCHAR(100),
  grade_level VARCHAR(20),
  capacity INTEGER DEFAULT 30,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE classroom_students (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id),
  student_id UUID NOT NULL REFERENCES users(id),
  joined_at TIMESTAMP DEFAULT NOW(),
  UNIQUE(classroom_id, student_id)
);

CREATE TABLE intervention_alerts (
  id UUID PRIMARY KEY,
  classroom_id UUID NOT NULL REFERENCES classrooms(id),
  student_id UUID NOT NULL REFERENCES users(id),
  alert_type VARCHAR(50), -- "struggling", "falling_behind", "inactive"
  skill_id UUID REFERENCES skills(id),
  message TEXT,
  severity VARCHAR(20), -- "low", "medium", "high"
  resolved BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP DEFAULT NOW()
);
```

### Frontend Pages (3,000 LOC new)

**Teacher Pages** (6 new pages):
1. **TeacherOnboardingPage** (250 LOC)
   - Sign up as teacher
   - Verify teacher credentials
   - Set up first classroom
   - Welcome tour

2. **TeacherDashboardPage** (300 LOC)
   - Overview of all classrooms
   - Class cards showing key metrics
   - Quick stats (students, mastery, activity)
   - Recent alerts
   - Create new classroom button

3. **ClassroomManagementPage** (350 LOC)
   - Classroom details & editing
   - Student roster with join/remove
   - Classroom settings (invite code)
   - Grade level, subject customization
   - Student join requests

4. **ClassProgressPage** (400 LOC)
   - Class-wide mastery pie chart
   - Per-skill performance heatmap
   - Individual student progress bars
   - Trend lines (class improving/declining)
   - Comparison to benchmarks

5. **InterventionAlertsPage** (250 LOC)
   - List of struggling students
   - Alert severity colors (red/yellow/green)
   - By skill (which skills are problematic)
   - By student (who needs help)
   - Dismiss/resolve alerts

6. **ClassAnalyticsPage** (450 LOC)
   - Detailed cohort analytics
   - Student distribution charts
   - Learning curve aggregations
   - Time-to-mastery statistics
   - Export class report as PDF

### Frontend Components (2,000 LOC new)

**Teacher-Specific Components**:
1. ClassroomCard (100 LOC) - Shows classroom summary
2. StudentProgressBar (100 LOC) - Individual student progress
3. SkillHeatmap (200 LOC) - Class skill performance matrix
4. AlertCard (100 LOC) - Intervention alert display
5. ClassroomForm (150 LOC) - Create/edit classroom
6. StudentRosterTable (200 LOC) - Manage students with add/remove
7. AnalyticsChart (200 LOC) - Time-series visualization
8. BenchmarkComparison (150 LOC) - Compare to average

### Custom Hooks (500 LOC new)

```typescript
// useClassroom.ts (100 LOC)
- getClassroom(classroomId)
- listClassrooms()
- createClassroom(data)
- updateClassroom(id, data)
- deleteClassroom(id)
- getStudents(classroomId)
- addStudent(classroomId, studentId)
- removeStudent(classroomId, studentId)

// useClassroomProgress.ts (150 LOC)
- getClassProgress(classroomId)
- getSkillStats(classroomId, skillId)
- getAlerts(classroomId)
- dismissAlert(alertId)

// useTeacherAnalytics.ts (150 LOC)
- getCohorMetrics(classroomId)
- generateReport(classroomId)
- compareToNational(classroomId)

// useInviteCode.ts (100 LOC)
- generateInviteCode(classroomId)
- validateInviteCode(code)
- joinViaInvite(code)
```

### Utilities (500 LOC new)

```typescript
// teacherValidators.ts (100 LOC)
- validateTeacherEmail()
- validateGradeLevel()
- validateSubject()
- validateClassroomName()

// reportGenerator.ts (200 LOC)
- generatePDFReport(classroomId)
- generateCSVExport(classroomId)
- formatPerformanceMetrics()
- createCharts()

// alertEngine.ts (200 LOC)
- detectStrugglingStudents(classroomId)
- calculateAlertSeverity()
- generateAlertMessage()
- trackAlertResolution()
```

---

## 🎯 WEEK 5 USER FLOWS

### Teacher Signup Flow
```
Sign up as Teacher
  ├─ Email verification
  ├─ Verify teaching credentials (license/ID)
  ├─ Create first classroom
  ├─ Invite students (via code or email)
  └─ Dashboard ready
```

### Classroom Management Flow
```
Create Classroom
  ├─ Set name, subject, grade level
  ├─ Generate invite code
  ├─ Students join via code
  ├─ Teacher approves pending joins (optional)
  └─ Class roster ready
```

### Progress Monitoring Flow
```
View Class Progress
  ├─ Class-wide mastery (pie chart)
  ├─ Per-skill heatmap (which students struggle where)
  ├─ Individual student progress (detailed bars)
  ├─ Trend detection (improving/stable/declining)
  └─ Benchmark comparison (vs national average)
```

### Intervention Alert Flow
```
System Detects Struggling Student
  ├─ P(Know) < 30% on skill for 5+ days
  ├─ Alert created (severity: high/medium/low)
  ├─ Teacher notified on dashboard
  ├─ Teacher clicks to see details
  │  ├─ Student learning curve
  │  ├─ Time-to-mastery projection
  │  ├─ Recommended intervention
  │  └─ Contact student option
  └─ Teacher resolves alert when student improves
```

---

## 📊 WEEK 5 DATA STRUCTURES

### Classroom Model
```typescript
interface Classroom {
  id: string;
  teacher_id: string;
  name: string;
  description?: string;
  subject: string;
  grade_level: string;
  capacity: number;
  invite_code: string;
  settings: {
    allow_peer_learning: boolean;
    show_leaderboard: boolean;
    parent_access: boolean;
    mastery_threshold: number; // default 0.85
  };
  created_at: string;
  updated_at: string;
}
```

### Student Roster Item
```typescript
interface ClassroomStudent {
  student_id: string;
  name: string;
  email: string;
  mastery_percent: number;
  skills_mastered: number;
  total_skills: number;
  current_skill: string;
  last_activity: string; // ISO timestamp
  status: 'active' | 'inactive' | 'struggling';
  joined_at: string;
}
```

### Intervention Alert
```typescript
interface InterventionAlert {
  id: string;
  classroom_id: string;
  student_id: string;
  student_name: string;
  alert_type: 'struggling' | 'falling_behind' | 'inactive';
  skill_id?: string;
  skill_name?: string;
  message: string;
  severity: 'low' | 'medium' | 'high';
  p_know: number;
  days_since_progress: number;
  recommendation: string;
  resolved: boolean;
  created_at: string;
  resolved_at?: string;
}
```

### Class Analytics
```typescript
interface ClassroomAnalytics {
  classroom_id: string;
  total_students: number;
  active_students: number;
  avg_mastery: number;
  mastery_distribution: {
    mastered: number;      // >= 85%
    developing: number;    // 30-85%
    beginner: number;      // < 30%
  };
  top_skills: Array<{
    skill_id: string;
    skill_name: string;
    mastery_percent: number;
    students_mastered: number;
  }>;
  struggling_skills: Array<{
    skill_id: string;
    skill_name: string;
    avg_mastery: number;
    students_struggling: number;
  }>;
  engagement: {
    exercises_completed: number;
    active_students_today: number;
    avg_time_per_session: number;
  };
  time_to_mastery: {
    avg_days: number;
    fastest: number;
    slowest: number;
  };
}
```

---

## 🔒 WEEK 5 SECURITY & PERMISSIONS

**Authorization Model**:
```typescript
Teacher can:
  ✅ Create classrooms
  ✅ Manage own classrooms
  ✅ View student progress in their classes
  ✅ Export class reports
  ✅ Create/manage alerts

Teacher CANNOT:
  ❌ View students from other teachers' classes
  ❌ Modify student account data
  ❌ Delete student accounts
  ❌ Access global admin functions

Student in Classroom:
  ✅ Can be seen by teacher
  ✅ Can see class leaderboard (if enabled)
  ✅ Can see own progress vs class avg
  ❌ Cannot see other students' detailed progress

Parent (future Week 6):
  ✅ Can view child's progress
  ✅ Can receive notifications
  ❌ Cannot access other students
```

---

## 🚀 WEEK 5 IMPLEMENTATION SEQUENCE

### Day 1: Backend (Teacher Service)
- [ ] Create teacher service (Go microservice)
- [ ] Implement classroom CRUD endpoints
- [ ] Implement student roster endpoints
- [ ] Implement alert generation logic
- [ ] Database migrations

### Day 2: Backend (Teacher Analytics)
- [ ] Implement progress aggregation
- [ ] Implement alert queries
- [ ] Implement report generation
- [ ] Cache class statistics

### Day 3-4: Frontend Pages
- [ ] TeacherOnboardingPage
- [ ] TeacherDashboardPage
- [ ] ClassroomManagementPage
- [ ] Wire to API endpoints

### Day 4-5: Frontend Analytics
- [ ] ClassProgressPage (charts)
- [ ] InterventionAlertsPage (alerts)
- [ ] ClassAnalyticsPage (detailed stats)
- [ ] Testing & polish

---

## 📈 WEEK 5 METRICS & GOALS

**Quality Targets**:
- ✅ 90%+ test coverage
- ✅ 0 console errors/warnings
- ✅ < 100ms API response times
- ✅ Accessible (WCAG AA)

**Performance Targets**:
- ✅ Classroom list loads < 500ms
- ✅ Class progress page loads < 1s
- ✅ Charts render smoothly (60fps)
- ✅ Real-time alert updates < 5s

**Feature Completeness**:
- ✅ All 6 pages functional
- ✅ All 8 components working
- ✅ All hooks implemented
- ✅ All alerts generating correctly
- ✅ Report export working

---

## 🔗 WEEK 5 API ADDITIONS

```bash
# Teacher Service on port 8005
POST   /v1/teachers/classrooms
GET    /v1/teachers/classrooms
GET    /v1/teachers/classrooms/:id
PUT    /v1/teachers/classrooms/:id
DELETE /v1/teachers/classrooms/:id

# Student roster management
POST   /v1/teachers/classrooms/:id/students
GET    /v1/teachers/classrooms/:id/students
DELETE /v1/teachers/classrooms/:id/students/:uid

# Analytics & monitoring
GET    /v1/teachers/classrooms/:id/progress
GET    /v1/teachers/classrooms/:id/skills/:sid
GET    /v1/teachers/classrooms/:id/alerts
POST   /v1/teachers/classrooms/:id/alerts/:aid/dismiss
POST   /v1/teachers/classrooms/:id/reports/export

# Invite codes
POST   /v1/classrooms/invite-codes
GET    /v1/classrooms/join/:code
POST   /v1/classrooms/join/:code
```

---

## 📊 WEEK 5 COMPLETION METRICS

| Component | LOC | Status |
|-----------|-----|--------|
| Teacher Service | 1,200 | 🚀 Ready |
| Database (3 tables) | 300 | 🚀 Ready |
| Teacher Pages (6) | 2,000 | 🚀 Ready |
| Components (8) | 2,000 | 🚀 Ready |
| Hooks (4) | 500 | 🚀 Ready |
| Utilities (3) | 500 | 🚀 Ready |
| Tests | 1,500 | 🚀 Ready |
| **TOTAL** | **8,000** | **🚀 READY** |

---

## ✨ WEEK 5 IMPACT

**For Teachers**:
- ✅ Complete classroom management
- ✅ Real-time progress monitoring
- ✅ Automated intervention alerts
- ✅ Data-driven insights
- ✅ Easy reporting for parents/admins

**For Students**:
- ✅ Continued learning unaffected
- ✅ Teacher-assigned classes optional
- ✅ Transparent progress visible to teacher

**For Administrators**:
- ✅ Teacher verification system
- ✅ Classroom management oversight
- ✅ Data compliance (FERPA ready)

---

## 📝 WEEK 5 DEPENDENCIES

**Requires (from Weeks 1-4)**:
- ✅ User Service (students)
- ✅ Personalization Service (P(Know) data)
- ✅ Progress Service (analytics)
- ✅ Frontend infrastructure

**Enables (for Weeks 6+)**:
- 🚀 Parent portal (Week 6)
- 🚀 Advanced notifications (Week 6)
- 🚀 Adaptive curriculum (Week 7)
- 🚀 Mobile features (Week 9)

---

## 🎓 WEEK 5: COMPLETE TEACHER PLATFORM

**By end of Week 5**:
- ✅ Teachers can create classrooms
- ✅ Teachers can manage rosters
- ✅ Teachers can see real-time progress
- ✅ Automated alerts for struggling students
- ✅ Analytics & reporting
- ✅ Invite codes for student enrollment

**Ready for Week 6**: Parent portal + advanced notifications

🚀 **PATHFINDER IS NOW A COMPLETE TEACHER PLATFORM.**

---

Generated: 2026-06-11  
Project: PATHFINDER Learning Platform  
Phase: 1 - Week 5 Planning  
Status: 🚀 READY TO BUILD
