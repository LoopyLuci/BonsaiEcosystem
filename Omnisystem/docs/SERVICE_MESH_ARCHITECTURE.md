# Omni-Service Mesh Controller

The Omni-Service Mesh Controller is the heart of the actor-based service fabric.

## Architecture

```
ServiceMeshController {
  bootstrap_actors: [ActorRef; 3],  // Replicated consensus group
  service_registry: CRDT<ServiceEntry>,
  actor_registry: CRDT<ActorEntry>,
  load_balancer: ConsistentHashRing<ActorRef>,
}

ServiceEntry {
  name: String,
  version: Version,
  endpoints: [ActorRef],
  health_status: HealthStatus,
  created_at: Timestamp,
}

ActorEntry {
  id: ActorId,
  service: String,
  node: NodeId,
  state: ActorState,
  restart_count: u32,
  created_at: Timestamp,
}

enum ActorState {
  Starting,
  Running,
  Paused,
  Stopping,
  Crashed(ErrorMessage),
  Recovered,
}
```

## Key Responsibilities

1. **Service Discovery** – maintain a CRDT of all active services and actors
2. **Actor Placement** – use consistent hashing + load information to place new actors
3. **Failure Detection** – heartbeat from each actor; mark crashed actors
4. **Supervision** – trigger restart policies when actors crash
5. **Load Balancing** – track node load; suggest migrations for overloaded nodes
6. **Observability Integration** – log all mesh events to build-observability

## Message Types

```
ServiceMeshMessage {
  | RegisterService(ServiceEntry),
  | DeregisterService(String),
  | RegisterActor(ActorEntry),
  | ActorCrashed(ActorId, Error),
  | RequestActorPlacement(ActorSpec) -> ActorRef,
  | HealthCheck(ActorId) -> HealthStatus,
  | MigrateActor(ActorId, TargetNode),
}
```

## Implementation Strategy

The controller is implemented in Aether as a replicated actor system. All state is stored in CRDTs so it automatically reconciles after network partitions.
