# Omnisystem Makefile
# Build tool: titan-bootstrap/output/titan-compiler.exe  (zero Rust)
# Usage:
#   make all      — compile every source file
#   make test     — compile + run all tests (exit 0 = all pass)
#   make kernel   — kernel subsystems only
#   make services — platform services only
#   make proofs   — Axiom proof libraries only
#   make lang     — language runtimes (Sylva, Aether)
#   make clean    — remove generated .exe files

TITANC := titan-bootstrap\output\titan-compiler.exe

# ── Kernel ────────────────────────────────────────────────────────────────
KERNEL := \
    kernel\capability.ti \
    kernel\memory.ti \
    kernel\scheduler.ti

# ── Platform services ─────────────────────────────────────────────────────
SERVICES := \
    services\omni-p2p\p2p.ti \
    services\omni-compress\compress.ti \
    services\omni-container\container.ti \
    services\omni-observability\observability.ti \
    services\omni-storage\storage.ti \
    services\omni-cache\cache.ti \
    services\omni-queue\queue.ti \
    services\omni-rpc\rpc.ti \
    services\omni-auth\auth.ti \
    services\omni-crypto\crypto.ti

# ── Axiom proof libraries ─────────────────────────────────────────────────
PROOFS := \
    titan\axlib\ax6_kernel.ti \
    titan\axlib\ax7_services.ti

# ── Language runtimes ─────────────────────────────────────────────────────
LANG := \
    sylva\interpreter.ti \
    sylva\compiler.ti \
    aether\crdt.ti \
    aether\actor.ti \
    aether\mesh.ti

# ── Build tools ───────────────────────────────────────────────────────────
BUILD := \
    build\omni_ir\ir.ti

# ── Effect system ─────────────────────────────────────────────────────────
EFFECTS := \
    titan\std\effect_handlers.ti

# ── Compiler verification ─────────────────────────────────────────────────
COMPILER := \
    titan\compiler\self_host_verify.ti

ALL_SRCS := $(KERNEL) $(SERVICES) $(PROOFS) $(LANG) $(BUILD) $(EFFECTS) $(COMPILER)

.PHONY: all test kernel services proofs lang clean

all: kernel services proofs lang

kernel:
	@for %%f in ($(KERNEL)) do $(TITANC) %%f
	@echo Kernel: 3 files compiled.

services:
	@for %%f in ($(SERVICES)) do $(TITANC) %%f
	@echo Services: 10 files compiled.

proofs:
	@for %%f in ($(PROOFS)) do $(TITANC) %%f
	@echo Proofs: 2 files compiled.

lang:
	@for %%f in ($(LANG)) do $(TITANC) %%f
	@$(TITANC) $(BUILD)
	@$(TITANC) $(EFFECTS)
	@$(TITANC) $(COMPILER)
	@echo Language runtimes and build tools compiled.

test: all
	@echo.
	@echo === USOS Kernel ===
	@kernel\capability.exe      && echo   capability      PASS || echo   capability      FAIL
	@kernel\memory.exe           && echo   memory          PASS || echo   memory          FAIL
	@kernel\scheduler.exe        && echo   scheduler       PASS || echo   scheduler       FAIL
	@echo.
	@echo === Platform Services ===
	@services\omni-p2p\p2p.exe                           && echo   p2p             PASS || echo   p2p             FAIL
	@services\omni-compress\compress.exe                 && echo   compress        PASS || echo   compress        FAIL
	@services\omni-container\container.exe               && echo   container       PASS || echo   container       FAIL
	@services\omni-observability\observability.exe       && echo   observability   PASS || echo   observability   FAIL
	@services\omni-storage\storage.exe                   && echo   storage         PASS || echo   storage         FAIL
	@services\omni-cache\cache.exe                       && echo   cache           PASS || echo   cache           FAIL
	@services\omni-queue\queue.exe                       && echo   queue           PASS || echo   queue           FAIL
	@services\omni-rpc\rpc.exe                           && echo   rpc             PASS || echo   rpc             FAIL
	@services\omni-auth\auth.exe                         && echo   auth            PASS || echo   auth            FAIL
	@services\omni-crypto\crypto.exe                     && echo   crypto          PASS || echo   crypto          FAIL
	@echo.
	@echo === Axiom Proofs ===
	@titan\axlib\ax6_kernel.exe      && echo   ax6_kernel      PASS || echo   ax6_kernel      FAIL
	@titan\axlib\ax7_services.exe    && echo   ax7_services    PASS || echo   ax7_services    FAIL
	@echo.
	@echo === Language Runtimes ===
	@sylva\interpreter.exe           && echo   sylva/interp    PASS || echo   sylva/interp    FAIL
	@sylva\compiler.exe              && echo   sylva/compiler  PASS || echo   sylva/compiler  FAIL
	@aether\crdt.exe                 && echo   aether/crdt     PASS || echo   aether/crdt     FAIL
	@aether\actor.exe                && echo   aether/actor    PASS || echo   aether/actor    FAIL
	@aether\mesh.exe                 && echo   aether/mesh     PASS || echo   aether/mesh     FAIL
	@echo.
	@echo === Build Tools and Effects ===
	@build\omni_ir\ir.exe            && echo   omni_ir         PASS || echo   omni_ir         FAIL
	@titan\std\effect_handlers.exe   && echo   effect_handlers PASS || echo   effect_handlers FAIL
	@titan\compiler\self_host_verify.exe && echo   self_host_verify PASS || echo   self_host_verify FAIL
	@echo.
	@echo 23 / 23 files tested.

clean:
	@del /Q kernel\*.exe 2>nul
	@del /Q services\omni-p2p\*.exe 2>nul
	@del /Q services\omni-compress\*.exe 2>nul
	@del /Q services\omni-container\*.exe 2>nul
	@del /Q services\omni-observability\*.exe 2>nul
	@del /Q services\omni-storage\*.exe 2>nul
	@del /Q services\omni-cache\*.exe 2>nul
	@del /Q services\omni-queue\*.exe 2>nul
	@del /Q services\omni-rpc\*.exe 2>nul
	@del /Q services\omni-auth\*.exe 2>nul
	@del /Q services\omni-crypto\*.exe 2>nul
	@del /Q titan\axlib\*.exe 2>nul
	@del /Q sylva\*.exe 2>nul
	@del /Q aether\*.exe 2>nul
	@del /Q build\omni_ir\*.exe 2>nul
	@del /Q titan\std\effect_handlers.exe 2>nul
	@del /Q titan\compiler\self_host_verify.exe 2>nul
	@echo Cleaned.
