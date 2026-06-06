# verify_phases75_82.ps1 — Integration & Communication Suite verification script

Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  VERIFICATION - PHASES 75-82 INTEGRATION & COMMUNICATION     ║" -ForegroundColor Cyan
Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$newModules = @(
    # Phase 75 OmniTranslate
    "titan/omnitranslate/translate_config.ti",
    "titan/omnitranslate/translation_pipeline.ti",
    "aether/omnitranslate/translation_worker.ae",
    "aether/omnitranslate/cache_manager.ae",
    "sylva/omnitranslate/translate_console.sy",
    "sylva/omnitranslate/glossary_editor.sy",
    "axiom/omnitranslate/translate_proofs.ax",
    "tests/test_omnitranslate_complete.ti",
    # Phase 76 OmniSpeech
    "titan/omnispeech/speech_config.ti",
    "titan/omnispeech/speech_engine.ti",
    "aether/omnispeech/audio_processor.ae",
    "aether/omnispeech/stream_manager.ae",
    "sylva/omnispeech/voice_dashboard.sy",
    "sylva/omnispeech/transcription_viewer.sy",
    "axiom/omnispeech/speech_proofs.ax",
    "tests/test_omnispeech_complete.ti",
    # Phase 77 OmniVideo
    "titan/omnivideo/video_config.ti",
    "titan/omnivideo/transcoder.ti",
    "aether/omnivideo/encoding_worker.ae",
    "aether/omnivideo/streaming_server.ae",
    "sylva/omnivideo/video_dashboard.sy",
    "sylva/omnivideo/media_library.sy",
    "axiom/omnivideo/video_proofs.ax",
    "tests/test_omnivideo_complete.ti",
    # Phase 78 OmniEmail
    "titan/omniemail/email_config.ti",
    "titan/omniemail/template_engine.ti",
    "aether/omniemail/email_sender.ae",
    "aether/omniemail/bounce_handler.ae",
    "sylva/omniemail/campaign_dashboard.sy",
    "sylva/omniemail/template_editor.sy",
    "axiom/omniemail/email_proofs.ax",
    "tests/test_omniemail_complete.ti",
    # Phase 79 OmniSMS
    "titan/omnisms/sms_config.ti",
    "titan/omnisms/notification_engine.ti",
    "aether/omnisms/delivery_worker.ae",
    "aether/omnisms/channel_router.ae",
    "sylva/omnisms/notification_dashboard.sy",
    "sylva/omnisms/template_builder.sy",
    "axiom/omnisms/sms_proofs.ax",
    "tests/test_omnisms_complete.ti",
    # Phase 80 OmniPayment
    "titan/omnipayment/payment_config.ti",
    "titan/omnipayment/transaction_engine.ti",
    "aether/omnipayment/payment_worker.ae",
    "aether/omnipayment/fraud_detector.ae",
    "sylva/omnipayment/revenue_dashboard.sy",
    "sylva/omnipayment/checkout_builder.sy",
    "axiom/omnipayment/payment_proofs.ax",
    "tests/test_omnipayment_complete.ti",
    # Phase 81 OmniInvoice
    "titan/omniinvoice/invoice_config.ti",
    "titan/omniinvoice/invoice_generator.ti",
    "aether/omniinvoice/invoice_worker.ae",
    "aether/omniinvoice/payment_tracker.ae",
    "sylva/omniinvoice/invoice_dashboard.sy",
    "sylva/omniinvoice/invoice_preview.sy",
    "axiom/omniinvoice/invoice_proofs.ax",
    "tests/test_omniinvoice_complete.ti",
    # Phase 82 OmniSupport
    "titan/omnisupport/support_config.ti",
    "titan/omnisupport/ticket_engine.ti",
    "aether/omnisupport/ticket_router.ae",
    "aether/omnisupport/chat_agent.ae",
    "sylva/omnisupport/help_desk.sy",
    "sylva/omnisupport/knowledge_base.sy",
    "axiom/omnisupport/support_proofs.ax",
    "tests/test_omnisupport_complete.ti"
)

$regression = @(
    "titan/omniweb/web_config.ti",
    "tests/test_omniweb_complete.ti"
)

$all = $newModules + $regression
$pass = 0
$fail = 0

foreach ($mod in $all) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "✓ $mod" -ForegroundColor Green
        $pass++
    } else {
        Write-Host "✗ $mod" -ForegroundColor Red
        $fail++
    }
}

Write-Host ""
Write-Host "════════════════════════════════════════════════════════════════"
Write-Host "Results: $pass passed, $fail failed out of $($all.Count) total" -ForegroundColor $(if ($fail -eq 0) { "Green" } else { "Red" })

if ($fail -eq 0) {
    Write-Host "✓ All modules verified, zero regressions." -ForegroundColor Green
    Write-Host "Ready for commit." -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $fail module(s) failed verification." -ForegroundColor Red
    exit 1
}
