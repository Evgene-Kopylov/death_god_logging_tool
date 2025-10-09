#!/usr/bin/env python3
"""
Скрипт для тестирования записи println! в файлы логов
Запускает бинарник и проверяет созданные файлы логов
"""

import subprocess
import os
import time
import sys

def run_binary_and_check_logs():
    """Запускает бинарник и проверяет логи"""
    
    # Очищаем директорию logs перед запуском
    logs_dir = "logs"
    if os.path.exists(logs_dir):
        for file in os.listdir(logs_dir):
            file_path = os.path.join(logs_dir, file)
            if os.path.isfile(file_path):
                os.remove(file_path)
    else:
        os.makedirs(logs_dir)
    
    print("🚀 Запускаем бинарник...")
    
    # Запускаем бинарник
    binary_path = "./target/release/death_god_logging_tool"
    
    try:
        # Запускаем процесс
        process = subprocess.Popen(
            [binary_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        # Ждем завершения
        stdout, stderr = process.communicate()
        
        print(f"📋 Статус завершения: {process.returncode}")
        
        # Даем время на запись в файлы
        print("⏳ Ждем завершения записи в файлы...")
        time.sleep(1)
        
    except Exception as e:
        print(f"❌ Ошибка при запуске бинарника: {e}")
        return False
    
    # Проверяем созданные файлы
    print("\n📁 Проверяем файлы в директории logs/:")
    
    if not os.path.exists(logs_dir):
        print("❌ Директория logs/ не создана")
        return False
    
    files = os.listdir(logs_dir)
    if not files:
        print("❌ Файлы логов не созданы")
        return False
    
    print(f"✅ Найдено файлов: {len(files)}")
    for file in files:
        print(f"   - {file}")
    
    # Проверяем содержимое файлов
    print("\n📄 Проверяем содержимое файлов:")
    
    for file in files:
        file_path = os.path.join(logs_dir, file)
        print(f"\n📖 Файл: {file}")
        print("-" * 50)
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
                print(content)
                
                # Проверяем наличие ключевых элементов
                checks = [
                    ("LOG_LEVEL=trace", "Настройка логгера"),
                    ("ttt - записть в лог-файл", "TRACE лог"),
                    ("ddd - записть в лог-файл", "DEBUG лог"),
                    ("i - записть в лог-файл", "INFO лог"),
                    ("w - записть в лог-файл", "WARN лог"),
                    ("eee - записть в лог-файл", "ERROR лог"),
                    ("print line ...", "println! вывод"),
                    ("Паника!!!", "Паника")
                ]
                
                print("\n🔍 Проверка содержимого:")
                for check_text, description in checks:
                    if check_text in content:
                        print(f"   ✅ {description}: найдено")
                    else:
                        print(f"   ❌ {description}: не найдено")
                        
        except Exception as e:
            print(f"❌ Ошибка при чтении файла {file}: {e}")
    
    return True

if __name__ == "__main__":
    print("🧪 Тестирование записи println! в файлы логов")
    print("=" * 60)
    
    success = run_binary_and_check_logs()
    
    print("\n" + "=" * 60)
    if success:
        print("🎉 Тестирование завершено успешно!")
    else:
        print("💥 Тестирование завершено с ошибками!")
        sys.exit(1)
