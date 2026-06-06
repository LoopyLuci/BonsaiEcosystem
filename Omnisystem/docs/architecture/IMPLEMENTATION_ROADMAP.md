# Omnisystem Self-Hosting: Complete Implementation Roadmap

## Overview

This document specifies the remaining steps (3-6) after the Rust seed compiler and OmniCore interpreter are working.

**Timeline:**
- Step 1: Rust seed (Cranelift) — **COMPLETE**
- Step 2: OmniCore Rust interpreter — **COMPLETE**
- Step 3: Port compiler to Titan (4-6 weeks)
- Step 4: Build Aether actor runtime (1-2 weeks)
- Step 5: Build Sylva REPL + debugger (2-3 weeks)
- Step 6: Build Axiom proof kernel (2-3 weeks)

**Success Criteria:** By end of Step 6, the entire Omnisystem is self-hosting with:
- Titan compiler compiling itself
- All runtime layers (OmniCore, Aether, Sylva, Axiom) written in Omni languages
- Zero external dependencies beyond Rust bootstrap
- Full capability-based security enforcement
- Time-travel debugging support
- Formal verification capabilities

---

## Step 3: Port the Titan Compiler to Titan

### Goal

Rewrite the Rust seed compiler in Titan itself, producing bit-identical output. Once complete, the Rust seed can be retired and the system is self-hosting.

### Implementation Path

#### Phase 3a: Implement Titan Lexer (lexer.ti)

**File:** `titan/compiler/lexer.ti`

Translate `titan-bootstrap/src/lexer.rs` to Titan. Must tokenize Titan source correctly.

**Key Components:**

```titan
// Tokens
enum TokenKind {
    Keyword(String),
    Identifier(String),
    Number(i64),
    StringLit(String),
    Symbol(String),
    EOF,
}

struct Token {
    kind: TokenKind,
    line: i64,
    column: i64,
}

// Lexer
struct Lexer {
    source: String,
    pos: i64,
    line: i64,
    column: i64,
    tokens: Vec<Token>,
}

impl Lexer {
    fn new(source: String) -> Lexer { ... }
    fn tokenize() -> Vec<Token> { ... }
    fn next_char() -> Option<char> { ... }
    fn peek_char(offset: i64) -> Option<char> { ... }
    fn consume_identifier() -> String { ... }
    fn consume_number() -> i64 { ... }
    fn consume_string() -> String { ... }
}
```

**Test:** Must produce identical tokens to the Rust seed when tokenizing the same source.

```bash
# Compile lexer with Rust seed
cargo run --release -- titan/compiler/lexer.ti -o lexer.bin --run

# Should output:
#   Lexer complete: N tokens generated
```

#### Phase 3b: Implement Titan Parser (parser.ti)

**File:** `titan/compiler/parser.ti`

Implement recursive descent parser that produces identical AST to Rust seed.

**Key Components:**

```titan
// AST node types
enum AstKind {
    Module,
    FunctionDef,
    Block,
    BinaryOp,
    UnaryOp,
    FunctionCall,
    IfExpr,
    LoopExpr,
    VarDecl,
    Return,
    Identifier,
    Literal,
}

struct AstNode {
    kind: AstKind,
    value: String,
    children: Vec<AstNode>,
    ty: String,  // inferred type
}

struct Parser {
    tokens: Vec<Token>,
    pos: i64,
}

impl Parser {
    fn parse_module() -> AstNode { ... }
    fn parse_function() -> AstNode { ... }
    fn parse_expression() -> AstNode { ... }
    fn parse_binary_expr(precedence: i64) -> AstNode { ... }
    fn parse_primary() -> AstNode { ... }
    fn match_token(kind: TokenKind) -> bool { ... }
    fn consume() -> Token { ... }
}
```

**Test:** Compile parser with seed, feed it tokenized output from lexer:

```bash
cargo run --release -- titan/compiler/parser.ti --run

# Should parse Titan source and output AST
```

#### Phase 3c: Implement Borrow Checker (borrow_checker.ti)

**File:** `titan/compiler/borrow_checker.ti`

Implements lifetime analysis and borrow checking. Critical for memory safety.

**Key Components:**

```titan
struct Region {
    name: String,
    parent: Option<Box<Region>>,
    variables: HashMap<String, VariableInfo>,
}

struct VariableInfo {
    name: String,
    ty: String,
    borrowed: bool,
    mutable: bool,
    lifetime: String,
}

struct BorrowChecker {
    regions: Vec<Region>,
    errors: Vec<String>,
}

impl BorrowChecker {
    fn check_module(ast: AstNode) { ... }
    fn check_function(func: AstNode) { ... }
    fn check_expression(expr: AstNode) { ... }
    fn enter_region(name: String) { ... }
    fn exit_region() { ... }
    fn bind_variable(name: String, info: VariableInfo) { ... }
    fn borrow_variable(name: String, mutable: bool) -> bool { ... }
}
```

**Errors Detected:**
- Use-after-free (variable accessed after dropped)
- Double borrow (variable borrowed twice in same scope)
- Mutable borrow of immutable variable
- Reference to local variable escapes scope

#### Phase 3d: Implement Codegen (codegen.ti)

**File:** `titan/compiler/codegen.ti`

Generates Cranelift IR from AST. This is the critical stage that produces machine code.

**Key Components:**

```titan
struct CodeGenerator {
    module: CraneliftModule,
    functions: HashMap<String, FuncId>,
    variables: HashMap<String, Value>,
}

impl CodeGenerator {
    fn compile_module(ast: AstNode) { ... }
    fn compile_function(func: AstNode) -> FuncId { ... }
    fn compile_expression(expr: AstNode) -> Value { ... }
    fn compile_binary_op(op: String, lhs: Value, rhs: Value) -> Value { ... }
    fn finalize() { ... }
}
```

**Output:** Cranelift IR (which JIT-compiles to machine code)

#### Phase 3e: Implement Compiler Main (main.ti)

**File:** `titan/compiler/main.ti`

Entry point that coordinates all stages.

```titan
fn main(args: Vec<String>) -> i64 {
    let input_file = args.get(0);
    
    // Stage 1: Lex
    let source = read_file(input_file);
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    if lexer.has_errors() {
        for err in lexer.errors() {
            eprintln!("Lex error: {}", err);
        }
        return 1;
    }
    
    // Stage 2: Parse
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_module();
    
    if parser.has_errors() {
        for err in parser.errors() {
            eprintln!("Parse error: {}", err);
        }
        return 1;
    }
    
    // Stage 3: Borrow Check
    let mut checker = BorrowChecker::new();
    checker.check_module(ast);
    
    if checker.has_errors() {
        for err in checker.errors() {
            eprintln!("Borrow error: {}", err);
        }
        return 1;
    }
    
    // Stage 4: Codegen
    let mut cg = CodeGenerator::new();
    cg.compile_module(ast);
    cg.finalize();
    
    return 0;
}
```

### Verification Strategy

**Bit-Identical Verification:**

```bash
# Compile test.ti with Rust seed
cd titan-bootstrap
cargo run --release -- ../test.ti -o rust_seed.bin

# Compile Titan compiler with Rust seed
cargo run --release -- ../titan/compiler/main.ti -o titan_compiler.bin

# Use new Titan compiler to compile test.ti
./titan_compiler.bin test.ti -o titan_compiled.bin

# Compare checksums
sha256sum rust_seed.bin
sha256sum titan_compiled.bin

# Must be identical!
```

Once bit-identical output is verified, the Rust seed is no longer needed.

---

## Step 4: Build Aether Actor Runtime

### Goal

Implement the actor model runtime that executes on top of OmniCore. Enables distributed systems with fault tolerance.

### File Organization

```
aether/
├── src/
│   ├── lib.rs                    (Main runtime)
│   ├── actor.rs                  (Actor struct, spawning)
│   ├── mailbox.rs                (Message queue)
│   ├── supervisor.rs             (Supervision tree)
│   ├── crdt.rs                   (Grow-only counter)
│   └── tests.rs
└── Cargo.toml
```

### Implementation Specification

#### Core Actor Model

```rust
// aether/src/actor.rs

pub struct Actor {
    id: ActorId,
    mailbox: Mailbox,
    state: Box<dyn Any>,
    supervisor: Option<ActorId>,
    children: Vec<ActorId>,
    restart_count: u32,
    status: ActorStatus,
}

pub enum ActorStatus {
    Initializing,
    Running,
    Suspended,
    Failed(String),
    Terminated,
}

pub enum Message {
    UserMessage(Vec<u8>),
    ProcessMailbox,
    SignalFailure(String),
    ChildFailed(ActorId, String),
    Restart,
    GetStatus,
    ScheduleTask(i64),  // Priority
}

impl Actor {
    pub fn spawn(name: String) -> (ActorId, Actor);
    pub fn send_message(&mut self, msg: Message) -> Result<(), String>;
    pub fn process_mailbox(&mut self) -> Vec<Message>;
    pub fn on_failure(&mut self, error: String);
    pub fn restart(&mut self);
}
```

#### Mailbox

```rust
// aether/src/mailbox.rs

pub struct Mailbox {
    queue: VecDeque<Message>,
    max_size: usize,
}

impl Mailbox {
    pub fn new(max_size: usize) -> Self;
    pub fn push(&mut self, msg: Message) -> Result<(), String>;
    pub fn pop(&mut self) -> Option<Message>;
    pub fn is_empty(&self) -> bool;
    pub fn len(&self) -> usize;
}
```

#### Supervision Tree

```rust
// aether/src/supervisor.rs

pub struct SupervisionTree {
    root: ActorId,
    actors: HashMap<ActorId, ActorHandle>,
}

pub enum RestartStrategy {
    OneForOne,      // Restart only failed child
    OneForAll,      // Restart all children
    RestForOne,     // Restart failed + younger siblings
}

impl SupervisionTree {
    pub fn new(root_actor: Actor) -> Self;
    pub fn spawn_child(&mut self, parent: ActorId, child: Actor) -> ActorId;
    pub fn handle_failure(&mut self, failed_id: ActorId, strategy: RestartStrategy);
    pub fn get_status(&self, id: ActorId) -> ActorStatus;
}
```

#### CRDT: Grow-Only Counter

```rust
// aether/src/crdt.rs

pub struct GCounter {
    replica_id: String,
    counters: HashMap<String, u64>,
}

impl GCounter {
    pub fn new(replica_id: String) -> Self {
        GCounter {
            replica_id,
            counters: HashMap::new(),
        }
    }

    pub fn increment(&mut self, by: u64) {
        let current = self.counters.entry(self.replica_id.clone())
            .or_insert(0);
        *current += by;
    }

    pub fn value(&self) -> u64 {
        self.counters.values().sum()
    }

    pub fn merge(&self, other: &GCounter) -> GCounter {
        let mut merged = GCounter::new(self.replica_id.clone());
        
        // Take max from each replica
        for (replica, count) in self.counters.iter() {
            merged.counters.insert(replica.clone(), *count);
        }
        for (replica, count) in other.counters.iter() {
            let existing = merged.counters.get(replica).copied().unwrap_or(0);
            if count > &existing {
                merged.counters.insert(replica.clone(), *count);
            }
        }
        merged
    }
}
```

#### Integration with OmniCore

```rust
// aether/src/lib.rs

use omnicore::*;

pub struct AetherRuntime {
    omnicore: OmniCore,
    supervision_tree: SupervisionTree,
    actors: HashMap<ActorId, Actor>,
}

impl AetherRuntime {
    pub fn new(omnicore: OmniCore) -> Self {
        AetherRuntime {
            omnicore,
            supervision_tree: SupervisionTree::new(...),
            actors: HashMap::new(),
        }
    }

    pub fn spawn(&mut self, name: String) -> ActorId;
    pub fn send(&mut self, target: ActorId, msg: Message) -> Result<(), String>;
    pub fn run_step(&mut self) -> Result<(), String>;
    pub fn run_until_idle(&mut self) -> Result<(), String>;
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_spawn() {
        let mut runtime = AetherRuntime::new(OmniCore::new(CapTable::new()));
        let actor_id = runtime.spawn("test-actor".to_string());
        assert!(runtime.actors.contains_key(&actor_id));
    }

    #[test]
    fn test_message_passing() {
        let mut runtime = AetherRuntime::new(OmniCore::new(CapTable::new()));
        let actor_id = runtime.spawn("test".to_string());
        let result = runtime.send(actor_id, Message::UserMessage(vec![1, 2, 3]));
        assert!(result.is_ok());
    }

    #[test]
    fn test_gcounter_merge() {
        let mut c1 = GCounter::new("node1".to_string());
        let mut c2 = GCounter::new("node2".to_string());
        
        c1.increment(42);
        c2.increment(58);
        
        let merged = c1.merge(&c2);
        assert_eq!(merged.value(), 100);
    }

    #[test]
    fn test_supervision_restart() {
        let mut runtime = AetherRuntime::new(OmniCore::new(CapTable::new()));
        let supervisor = runtime.spawn("supervisor".to_string());
        let child = runtime.spawn("child".to_string());
        
        // Simulate child failure
        runtime.actors.get_mut(&child).unwrap().status = ActorStatus::Failed("test".to_string());
        
        // Supervisor should restart it
        runtime.run_step();
        
        let status = runtime.get_status(child);
        assert_eq!(status, ActorStatus::Running);
    }
}
```

---

## Step 5: Build Sylva Interactive Frontend

### Goal

Create a REPL and time-travel debugger for interactive exploration and debugging of Omnisystem programs.

### File Organization

```
sylva/
├── src/
│   ├── lib.rs                    (Main REPL)
│   ├── lexer.rs                  (Expression parser)
│   ├── evaluator.rs              (Expression evaluation)
│   ├── debugger.rs               (Time-travel debugging)
│   ├── types.rs                  (Gradual typing)
│   └── tests.rs
├── Cargo.toml
└── examples/
    └── repl_demo.rs
```

### Implementation Specification

#### REPL Structure

```rust
// sylva/src/lib.rs

pub struct Repl {
    evaluator: Evaluator,
    debugger: TimeTravel Debugger,
    type_env: TypeEnvironment,
    history: Vec<HistoryEntry>,
}

pub struct HistoryEntry {
    input: String,
    result: Result<Value, String>,
    timestamp: i64,
}

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Tuple(Vec<Value>),
    Function(String),
    Null,
}

impl Repl {
    pub fn new() -> Self;
    pub fn read_eval_print(&mut self, input: &str) -> Result<String, String>;
    pub fn get_history(&self) -> Vec<HistoryEntry>;
    pub fn rewind_to(&mut self, timestamp: i64) -> Result<(), String>;
}
```

#### Expression Evaluator

```rust
// sylva/src/evaluator.rs

pub struct Evaluator {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Function>,
}

pub struct Function {
    params: Vec<String>,
    body: Expr,
}

pub enum Expr {
    Literal(Value),
    Identifier(String),
    BinaryOp { op: String, lhs: Box<Expr>, rhs: Box<Expr> },
    UnaryOp { op: String, operand: Box<Expr> },
    FunctionCall { callee: String, args: Vec<Expr> },
    IfExpr { cond: Box<Expr>, then_branch: Box<Expr>, else_branch: Option<Box<Expr>> },
    LetBinding { name: String, value: Box<Expr>, body: Box<Expr> },
    Lambda { params: Vec<String>, body: Box<Expr> },
}

impl Evaluator {
    pub fn eval(&mut self, expr: &Expr) -> Result<Value, String>;
    pub fn eval_binary_op(&mut self, op: &str, lhs: Value, rhs: Value) -> Result<Value, String>;
    pub fn call_function(&mut self, name: &str, args: &[Value]) -> Result<Value, String>;
    pub fn define_function(&mut self, name: String, func: Function);
}
```

#### Time-Travel Debugger

```rust
// sylva/src/debugger.rs

pub struct TimeTravel Debugger {
    checkpoints: Vec<Checkpoint>,
    current: usize,
}

pub struct Checkpoint {
    timestamp: i64,
    state: EvaluatorState,
    locals: HashMap<String, Value>,
}

pub struct EvaluatorState {
    pc: usize,          // Program counter
    stack: Vec<Value>,
    call_stack: Vec<String>,
}

impl TimeTravel Debugger {
    pub fn new() -> Self;
    pub fn save_checkpoint(&mut self, state: EvaluatorState, locals: HashMap<String, Value>);
    pub fn step_forward(&mut self) -> Result<(), String>;
    pub fn step_backward(&mut self) -> Result<(), String>;
    pub fn rewind_to(&mut self, timestamp: i64) -> Result<(), String>;
    pub fn get_locals_at(&self, timestamp: i64) -> Option<HashMap<String, Value>>;
    pub fn get_call_stack_at(&self, timestamp: i64) -> Option<Vec<String>>;
}
```

#### Gradual Type Checking

```rust
// sylva/src/types.rs

pub enum Type {
    Dynamic,                           // Any type
    Integer,
    Float,
    String,
    Boolean,
    Tuple(Vec<Type>),
    Function { params: Vec<Type>, ret: Box<Type> },
    Union(Vec<Type>),
}

pub struct TypeEnvironment {
    bindings: HashMap<String, Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self;
    pub fn bind(&mut self, name: String, ty: Type);
    pub fn infer(&self, expr: &Expr) -> Result<Type, String>;
    pub fn check(&self, expr: &Expr, expected: &Type) -> Result<(), String>;
}
```

### Usage Examples

```rust
// Start REPL
let mut repl = Repl::new();

// Basic expressions
repl.read_eval_print("1 + 2")?;           // Result: 3
repl.read_eval_print("x = 42")?;          // Bind x
repl.read_eval_print("x * 2")?;           // Result: 84

// Function definition
repl.read_eval_print("fn add(a, b) { a + b }")?;
repl.read_eval_print("add(10, 20)")?;     // Result: 30

// Time-travel debugging
repl.read_eval_print("debug on")?;        // Enable checkpoints
repl.read_eval_print("x = 1")?;           // Checkpoint 1
repl.read_eval_print("x = x + 1")?;       // Checkpoint 2
repl.read_eval_print("x = x * 2")?;       // Checkpoint 3
repl.rewind_to(2)?;                       // Go back to after "x = x + 1"
repl.read_eval_print("x")?;               // Result: 2
```

---

## Step 6: Build Axiom Proof Kernel

### Goal

Implement dependent types and formal verification capabilities. The kernel is intentionally kept small (<500 LOC) and trusted.

### File Organization

```
axiom/
├── src/
│   ├── lib.rs                    (Kernel)
│   ├── terms.rs                  (AST representation)
│   ├── types.rs                  (Type checker)
│   ├── normalize.rs              (Normalization)
│   └── tests.rs
└── Cargo.toml
```

### Implementation Specification

#### Core Types

```rust
// axiom/src/terms.rs

pub enum Term {
    Var(u32),                                       // De Bruijn index
    Type(u32),                                      // Type_i
    Pi { param_ty: Box<Term>, body: Box<Term> },   // Π(x:A).B
    Lam { body: Box<Term> },                        // λx.B
    App { func: Box<Term>, arg: Box<Term> },       // M N
    Sigma { fst_ty: Box<Term>, snd_ty: Box<Term> }, // Σ(x:A).B
    Pair { fst: Box<Term>, snd: Box<Term> },       // (a, b)
    Let { value: Box<Term>, body: Box<Term> },     // let x = M in N
}

impl Term {
    pub fn is_closed(&self) -> bool;
    pub fn free_vars(&self) -> Vec<u32>;
    pub fn shift(&self, delta: i32) -> Term;       // Shift free vars by delta
    pub fn substitute(&self, var: u32, replacement: &Term) -> Term;
}
```

#### Type Checker (Bidirectional)

```rust
// axiom/src/types.rs

pub struct TypeChecker {
    context: Vec<(String, Term)>,  // Γ = [(x1, A1), (x2, A2), ...]
}

impl TypeChecker {
    pub fn new() -> Self;
    
    // Bidirectional checking
    pub fn check(&mut self, term: &Term, ty: &Term) -> Result<(), String>;
    pub fn infer(&mut self, term: &Term) -> Result<Term, String>;
    
    // Context management
    pub fn push_binding(&mut self, name: String, ty: Term);
    pub fn pop_binding(&mut self);
}

// Key rules:
// 
// Type formation (universe hierarchy):
//   Γ ⊢ Type_i : Type_{i+1}
//
// Pi formation:
//   Γ ⊢ A : Type_i    Γ, x:A ⊢ B : Type_j
//   ────────────────────────────────────
//   Γ ⊢ Π(x:A).B : Type_max(i,j)
//
// Lambda intro:
//   Γ, x:A ⊢ M : B
//   ─────────────────────
//   Γ ⊢ λx.M : Π(x:A).B
//
// Application elim:
//   Γ ⊢ M : Π(x:A).B    Γ ⊢ N : A
//   ────────────────────────────
//   Γ ⊢ M N : B[x := N]
```

#### Normalizer (WHNF)

```rust
// axiom/src/normalize.rs

pub fn normalize(term: &Term) -> Term {
    match term {
        // Beta reduction: (λx.B) N → B[x := N]
        Term::App { func, arg } => {
            if let Term::Lam { body } = normalize(func).as_ref() {
                let shifted_arg = arg.shift(1);  // Shift arg under binder
                return normalize(&body.substitute(0, &shifted_arg));
            }
            term.clone()
        }
        
        // Let inlining: let x = M in N → N[x := M]
        Term::Let { value, body } => {
            let shifted_value = value.shift(1);
            return normalize(&body.substitute(0, &shifted_value));
        }
        
        // Recursive normalization
        Term::Pi { param_ty, body } => {
            Term::Pi {
                param_ty: Box::new(normalize(param_ty)),
                body: Box::new(normalize(body)),
            }
        }
        
        _ => term.clone(),
    }
}

// Weak Head Normal Form (WHNF)
pub fn whnf(term: &Term) -> Term {
    normalize(term)
}
```

#### Trusted Core (Keep Small!)

```rust
// axiom/src/lib.rs
// The entire kernel is approximately 400-500 lines.

pub struct Kernel {
    type_checker: TypeChecker,
    definitions: HashMap<String, Term>,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel {
            type_checker: TypeChecker::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, term: Term, ty: Term) -> Result<(), String> {
        // Check term has type ty
        self.type_checker.check(&term, &ty)?;
        self.definitions.insert(name, term);
        Ok(())
    }

    pub fn check_theorem(&mut self, statement: Term) -> Result<(), String> {
        // Theorem statements are Type_0 = True
        let _normalized = normalize(&statement);
        // In a real system, we'd verify the proof here
        Ok(())
    }
}
```

### Example Theorems

```rust
// Theorem: Type_0 : Type_1
let type0 = Term::Type(0);
let type1 = Term::Type(1);
kernel.type_checker.infer(&type0)?;  // Should return Type_1

// Theorem: Identity function (λx.x) : Π(x:A).A
let identity = Term::Lam { 
    body: Box::new(Term::Var(0)) 
};
// Types as: ∀ A : Type_0. A → A

// Theorem: Dependent pair
let sigma = Term::Sigma {
    fst_ty: Box::new(Term::Type(0)),     // A : Type_0
    snd_ty: Box::new(Term::Var(0)),      // B : A → Type_0
};
// Types as: Type_0
```

---

## Integration: Building the Complete System

Once all steps are complete:

### Build Sequence

```bash
# Step 1: Build Rust seed (already done)
cd titan-bootstrap
cargo build --release

# Step 2: Build OmniCore interpreter
cd omnicore
cargo build --release

# Step 3: Compile Titan compiler with seed
cd titan-bootstrap
cargo run --release -- ../titan/compiler/main.ti -o titan_compiler.bin

# Step 4: Build Aether (Rust crate)
cd aether
cargo build --release

# Step 5: Build Sylva (Rust crate)
cd sylva
cargo build --release

# Step 6: Build Axiom (Rust crate)
cd axiom
cargo build --release

# Integration test
cd omnisystem_ide
cargo run --release -- --init
```

### Final Verification

```bash
# Verify bit-identical compilation
./titan_compiler.bin test.ti -o output1.bin
cargo run --release -- ../test.ti -o output2.bin
sha256sum output1.bin output2.bin  # Must be identical

# Run full integration test
cd tests
cargo test --release -- --nocapture

# Launch IDE
../omnisystem_ide/target/release/studio
```

---

## Success Criteria

### Step 3: Compiler Bootstrap
- ✓ Rust seed compiles Titan source
- ✓ Titan compiler compiles itself
- ✓ Output is bit-identical to Rust seed
- ✓ Rust seed can be deleted

### Step 4: Aether Runtime
- ✓ Actors spawn and execute
- ✓ Message passing works
- ✓ Supervision tree restarts failed actors
- ✓ GCounter CRDTs converge correctly

### Step 5: Sylva REPL
- ✓ Expressions evaluate correctly
- ✓ Functions can be defined and called
- ✓ Time-travel debugging works
- ✓ REPL integrates with Titan code

### Step 6: Axiom Proof Kernel
- ✓ Type checker accepts valid terms
- ✓ Type checker rejects invalid terms
- ✓ Normalizer computes correctly
- ✓ Kernel is <500 LOC

### Full System
- ✓ All layers integrated
- ✓ Zero external dependencies (except Rust bootstrap)
- ✓ Full self-hosting achieved
- ✓ Omnisystem IDE operational

---

## Timeline Estimate

| Step | Duration | Difficulty |
|------|----------|------------|
| 1: Rust seed (Cranelift) | 1-2 days | Medium |
| 2: OmniCore interpreter | 3-5 days | Medium |
| 3: Titan compiler rewrite | 4-6 weeks | High |
| 4: Aether actor runtime | 1-2 weeks | Medium |
| 5: Sylva REPL + debugger | 2-3 weeks | Medium |
| 6: Axiom proof kernel | 2-3 weeks | High |
| **Total** | **~10-13 weeks** | |

---

## References

- **Dependent Types:** https://en.wikipedia.org/wiki/Dependent_type
- **De Bruijn Indices:** https://en.wikipedia.org/wiki/De_Bruijn_index
- **WHNF Normalization:** https://en.wikipedia.org/wiki/Weak_head_normal_form
- **Actor Model:** https://en.wikipedia.org/wiki/Actor_model
- **CRDTs:** https://crdt.tech/
- **Time-Travel Debugging:** https://en.wikipedia.org/wiki/Time_travel_debugging
- **Bootstrap Compilers:** https://en.wikipedia.org/wiki/Bootstrapping_(compilers)
