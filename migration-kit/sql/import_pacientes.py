
import csv
import sqlite3
import os

# Caminhos dos arquivos
CSV_PATH = os.path.join(os.path.dirname(__file__), 'tabela_pacientes.csv')
DB_PATH = os.path.join(os.path.dirname(__file__), '../../elevve_clinic_deva_db.db')  # Ajuste se necessário

# Conexão com o banco
conn = sqlite3.connect(DB_PATH)
cur = conn.cursor()

with open(CSV_PATH, encoding='utf-8') as csvfile:
    reader = csv.DictReader(csvfile, delimiter=';')
    for row in reader:
        full_name = row['Paciente']
        cpf = row['CPF'] if row['CPF'] != '-' else None
        phone = row['Telefone'] if row['Telefone'] != '-' else None
        email = row['Email'] if row['Email'] != '-' else None
        birth_date = row['Nascimento'] if row['Nascimento'] != '-' else None
        city = row['Cidade'] if row['Cidade'] != '-' else None
        labels = row['Etiquetas'] if row['Etiquetas'] != '-' else None

        # Insere apenas se tiver email ou telefone
        if email or phone:
            cur.execute("""
                INSERT INTO deva_elevveclinic_users
                (email, full_name, phone, birth_date, city, labels, cpf, role, is_active)
                VALUES (?, ?, ?, ?, ?, ?, ?, 'client', 1)
            """, (email, full_name, phone, birth_date, city, labels, cpf))

conn.commit()
conn.close()
print('Importação concluída.')
