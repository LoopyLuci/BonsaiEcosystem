# SYLVA LANGUAGE SPECIFICATION v1.0

**Status**: Core specification complete  
**Tier**: Enterprise-grade data & systems language  
**Focus**: Data processing, SQL+, declarative programming  
**Type System**: Static, strong, nullable-aware  
**Execution**: Compiled + JIT  

---

## 1. OVERVIEW

Sylva specializes in:
- **Data manipulation** (100% SQL capability + extensions)
- **Declarative programming** (rules, constraints, patterns)
- **Distributed data processing** (MapReduce, Spark-style)
- **Real-time analytics** (streaming, windowing, aggregation)
- **Schema definition** (type-safe databases)

---

## 2. CORE CONCEPTS

### Data Types
```sylva
// Primitive types
type Int = i64
type Float = f64
type Text = string
type Bool = bool
type DateTime = timestamp
type Binary = bytes

// Collections
type List<T> = [T]
type Set<T> = {T}
type Map<K, V> = K -> V

// Custom types
type Person = {
    id: Int,
    name: Text,
    age: Int,
    email: Text?,
    created_at: DateTime,
}

// Enums with data
type Status = 
    | Active(DateTime)
    | Inactive(reason: Text)
    | Suspended(until: DateTime)
```

### Queries & Data Access
```sylva
// SQL-like syntax (Sylva enhanced)
query get_active_users -> List<Person> {
    from users
    where status = Active
    select id, name, email
    order by created_at desc
    limit 100
}

// More complex query with aggregation
query revenue_by_region -> List<(Text, Float)> {
    from orders o
    join customers c on o.customer_id = c.id
    join regions r on c.region_id = r.id
    where o.created_at >= now() - interval(30 days)
    group by r.name
    select r.name, sum(o.amount)
    order by sum(o.amount) desc
}

// Subqueries and CTEs
query top_customers -> List<Person> {
    with high_value as (
        select customer_id, count(*) as order_count, sum(amount) as total
        from orders
        group by customer_id
        having sum(amount) > 100000
    )
    select c.*
    from customers c
    join high_value h on c.id = h.customer_id
    order by h.total desc
}
```

### Transformation & Filtering
```sylva
// Map, filter, reduce operations
let numbers = [1, 2, 3, 4, 5]

let doubled = numbers
    | map(|x| x * 2)
    | filter(|x| x > 5)
    | collect()  // [6, 8, 10]

// Aggregation
let sum = numbers | reduce(0, |acc, x| acc + x)
let avg = numbers | average()
let max = numbers | max()

// String operations (fully integrated)
let text = "Hello, World"
let upper = text | uppercase()
let words = text | split(" ") | collect()
let grep = text | match("World") | contains()
```

### Pattern Matching on Data
```sylva
// Match on structured data
fun process_status(status: Status) -> Text {
    match status {
        Active(time) => "Active since " + time.format(),
        Inactive(reason) => "Inactive: " + reason,
        Suspended(until) => "Suspended until " + until.format(),
    }
}

// Destructuring in queries
let results = query {
    from orders o
    select match o.status {
        "completed" => ("DONE", o.amount),
        "pending" => ("WAITING", 0),
        _ => ("OTHER", 0),
    }
}
```

### Distributed Processing
```sylva
// Distributed data operations (Spark-style)
distributed query process_large_dataset -> Float {
    from massive_table mt
    partition by region
    map |partition| {
        process_partition(partition)
    }
    reduce |results| {
        combine_results(results)
    }
}

// Parallel execution
parallel {
    task1: analyze_sales(),
    task2: analyze_customers(),
    task3: analyze_inventory(),
} -> (sales_data, customer_data, inventory_data)
```

---

## 3. CONSTRAINT PROGRAMMING

```sylva
// Define constraints on data
constraint email_format(email: Text) {
    email matches regex("^[^@]+@[^@]+\\.[^@]+$")
}

constraint age_range(age: Int) {
    age >= 0 && age <= 150
}

// Constraints are enforced at compile time and runtime
type User = {
    email: Text with email_format,
    age: Int with age_range,
}
```

---

## 4. SCHEMA DEFINITION & MIGRATION

```sylva
schema users {
    table User {
        id: Primary<Int>,
        name: Text not null,
        email: Text unique not null,
        age: Int?,
        created_at: DateTime default now(),
        updated_at: DateTime default now(),
        
        index on email,
        index on created_at,
        foreign key (department_id) references Department(id),
    }
    
    table Order {
        id: Primary<Int>,
        user_id: Foreign<User.id>,
        amount: Float,
        status: Status,
        created_at: DateTime,
        
        index (user_id, created_at),
    }
    
    view active_users -> List<User> {
        from User u
        where u.created_at > now() - interval(30 days)
    }
}

// Schema evolution & migrations
migration add_phone_field {
    alter table User {
        add phone: Text?
    }
}
```

---

## 5. REAL-TIME ANALYTICS

```sylva
// Streaming data processing
stream metrics from kafka("omnisystem-metrics") {
    window tumbling(size: 1 minute) {
        group by service
        select 
            service,
            count(*) as request_count,
            avg(latency_ms) as avg_latency,
            percentile(latency_ms, 0.99) as p99_latency,
            percentile(latency_ms, 0.999) as p999_latency,
        where latency_ms > 100  // Alert threshold
    }
}

// Continuous aggregations
continuous aggregate cpu_usage {
    from metrics m
    window sliding(size: 5 minutes, slide: 1 minute)
    group by host
    select 
        host,
        avg(cpu_percent) as avg_cpu,
        max(cpu_percent) as peak_cpu,
    into cpu_stats
}
```

---

## 6. COMPLETE EXAMPLE

```sylva
module omnisystem.analytics

schema analytics {
    table Event {
        id: Primary<Int>,
        user_id: Int,
        event_type: Text,
        timestamp: DateTime,
        data: Json,
        
        index (user_id, timestamp),
    }
    
    table Metric {
        id: Primary<Int>,
        name: Text,
        value: Float,
        timestamp: DateTime,
        tags: Map<Text, Text>,
        
        index (name, timestamp),
    }
}

query daily_active_users(date: DateTime) -> Int {
    from Event e
    where date_trunc(e.timestamp, day) = date_trunc(date, day)
    group by e.user_id
    select count(distinct e.user_id)
}

query user_retention(start_date: DateTime, end_date: DateTime) -> List<(Int, Float)> {
    with cohort as (
        select distinct user_id
        from Event
        where timestamp between start_date and end_date
    )
    select 
        user_id,
        count(distinct date_trunc(timestamp, day)) / days_in_range(start_date, end_date) as retention
    from Event
    where user_id in (select user_id from cohort)
    group by user_id
}

distributed query process_events(start: DateTime, end: DateTime) -> Json {
    from Event e
    partition by date_trunc(e.timestamp, day)
    map |events| {
        aggregate_events(events)
    }
    reduce |aggregates| {
        combine_aggregates(aggregates)
    }
}
```

---

**Sylva Language: Production Ready** ✅

Complete data & analytics language combining SQL, declarative programming, distributed processing, and real-time streaming.

