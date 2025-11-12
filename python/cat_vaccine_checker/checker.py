import sqlite3 

try:
    conn = sqlite3.connect('sweigartcats.db', isolation_level=None)
    print('Connected to the database')
    sql_query = 'SELECT * FROM sqlite_master'

    cursor = conn.cursor()
    cursor.execute(sql_query)
    print(cursor.fetchall())
except sqlite3.Error as error:
    print('Failed to executed the statements', error)
    
finally:
    if conn:
        conn.close()
        print("the sqlite connection is closed")
