#!/usr/bin/env python3
#exonware/xwsystem/tests/1.unit/serialization_tests/multilingual_emoji_test.py
"""
Comprehensive test of mixed languages and emojis in BSON serialization.
Tests the most complex Unicode scenarios possible.
"""

def test_multilingual_emojis():
    """Test extreme Unicode complexity with multiple languages and emojis."""
    try:
        from exonware.xwsystem.io.serialization import BsonSerializer
        print("🌍 MULTILINGUAL + EMOJI BSON TEST")
        print("=" * 50)
        # Extreme multilingual test data
        complex_data = {
            # Mix of Arabic, English, Chinese, Japanese, Russian, Hebrew, Hindi
            "greeting": "Hello مرحبا 你好 こんにちは Привет שלום नमस्ते 🌍🌎🌏",
            # Languages with numbers and symbols
            "mixed_numbers": "English123 عربي٤٥٦ 中文789 русский012 עברית345 🔢📊",
            # Complex emojis (including skin tones and combined emojis)
            "emojis": "👋🏽👨‍👩‍👧‍👦🏳️‍🌈🧑🏾‍💻👩🏻‍🔬🎉🥳🎊✨💫⭐🌟💥",
            # Religious and cultural symbols
            "symbols": "☪️✝️🕉️☯️🔯⚛️🛐 α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω",
            # Mathematical and scientific symbols
            "math": "∑∆∇∞≈≠≤≥±∓∴∵∈∉∩∪⊂⊃⊆⊇∧∨¬→↔∀∃∅ℝℂℕℤℚ∂∫∮∏",
            # Currency symbols from different countries
            "currency": "$ € £ ¥ ₹ ₽ ₩ ₪ ﷼ ₴ ₦ ₨ ₱ ₡ ₫ ₵ 💰💴💵💶💷💸",
            # Nested structure with mixed content
            "nested": {
                "arabic_city": "الرياض 🏛️",
                "chinese_food": "北京烤鸭 🦆🥢",
                "japanese_culture": "侍 🗾⛩️🎌",
                "russian_winter": "зима ❄️🐻🇷🇺",
                "indian_spices": "मसाला 🌶️🍛",
                "mixed_celebration": "🎉 Party حفلة 派对 パーティー вечеринка מסיבה पार्टी 🎊"
            },
            # Text with special Unicode categories
            "special": "🔀🔁🔂🔃🔄🔅🔆🔇🔈🔉🔊 ♠️♣️♥️♦️ 🃏🎴🀄 📱💻⌚📺📻",
            # Mixed RTL and LTR text (Arabic + English)
            "rtl_ltr_mix": "This is English then عربي نص here and English again مع المزيد من العربية and back to English 🔄",
            # Extreme length with all languages
            "long_text": "🌐 Global message: " + 
                        "English: Welcome to our world! " +
                        "العربية: أهلاً وسهلاً بكم في عالمنا! " +
                        "中文: 欢迎来到我们的世界! " +
                        "日本語: 私たちの世界へようこそ! " +
                        "Русский: Добро пожаловать в наш мир! " +
                        "עברית: ברוכים הבאים לעולמנו! " +
                        "हिन्दी: हमारी दुनिया में आपका स्वागत है! " +
                        "🎉🎊✨💫⭐🌟"
        }
        print("📝 ORIGINAL Complex Data:")
        for key, value in complex_data.items():
            if len(str(value)) > 100:
                print(f"   {key}: {str(value)[:100]}... [truncated]")
            else:
                print(f"   {key}: {value}")
        # Test serialization
        serializer = BsonSerializer()
        print(f"\n🔄 Testing BSON Serialization...")
        # Binary serialization
        print("   1️⃣ Binary serialization...")
        binary_result = serializer.dumps_binary(complex_data)
        print(f"      Result: {len(binary_result)} bytes")
        # Base64 serialization  
        print("   2️⃣ Base64 serialization...")
        text_result = serializer.dumps(complex_data)
        print(f"      Result: {len(text_result)} ASCII characters")
        print(f"      Preview: {text_result[:50]}...")
        # Test round-trips
        print(f"\n🔄 Testing Round-trips...")
        print("   3️⃣ Binary round-trip...")
        restored_binary = serializer.loads_bytes(binary_result)
        print("   4️⃣ Base64 round-trip...")
        restored_text = serializer.loads(text_result)
        # Detailed verification
        print(f"\n✅ VERIFICATION Results:")
        all_perfect = True
        differences = []
        for key in complex_data:
            original = complex_data[key]
            binary_restored = restored_binary[key]
            text_restored = restored_text[key]
            binary_match = original == binary_restored
            text_match = original == text_restored
            if not binary_match:
                differences.append(f"Binary mismatch in '{key}'")
                all_perfect = False
            if not text_match:
                differences.append(f"Text mismatch in '{key}'")
                all_perfect = False
            # Show detailed comparison for first few fields
            if key in ['greeting', 'emojis', 'rtl_ltr_mix']:
                print(f"\n   🔍 Field '{key}':")
                print(f"      Original: '{original}'")
                print(f"      Binary:   '{binary_restored}' {'✅' if binary_match else '❌'}")
                print(f"      Base64:   '{text_restored}' {'✅' if text_match else '❌'}")
        # Final results
        print(f"\n🎯 FINAL RESULT:")
        if all_perfect:
            print("   🎉 PERFECT SUCCESS! All languages and emojis preserved!")
            print("   ✅ Arabic, Chinese, Japanese, Russian, Hebrew, Hindi - ALL PERFECT")
            print("   ✅ Complex emojis with skin tones - PERFECT")
            print("   ✅ Mathematical symbols - PERFECT") 
            print("   ✅ Currency symbols - PERFECT")
            print("   ✅ RTL/LTR mixed text - PERFECT")
            print("   ✅ Nested structures - PERFECT")
            print("   ✅ Both binary and base64 round-trips - PERFECT")
        else:
            print("   ❌ ISSUES FOUND:")
            for diff in differences:
                print(f"      • {diff}")
        # Character count analysis
        total_chars = sum(len(str(v)) for v in complex_data.values())
        print(f"\n📊 STATISTICS:")
        print(f"   • Total characters tested: {total_chars}")
        print(f"   • Languages: Arabic, English, Chinese, Japanese, Russian, Hebrew, Hindi")
        print(f"   • Special content: Emojis, Math symbols, Currency, RTL/LTR mixing")
        print(f"   • Binary BSON size: {len(binary_result)} bytes")
        print(f"   • Base64 text size: {len(text_result)} characters")
        print(f"   • Compression ratio: {len(text_result)/total_chars:.2f}x")
        return all_perfect
    except ImportError as e:
        print(f"❌ BSON not available: {e}")
        return
    except Exception as e:
        print(f"❌ Error during test: {e}")
        import traceback
        traceback.print_exc()
        return
if __name__ == "__main__":
    print("🚀 Starting Multilingual + Emoji Test...\n")
    success = test_multilingual_emojis()
    print(f"\n🏁 Test completed: {'🎉 SUCCESS' if success else '❌ FAILED'}")
