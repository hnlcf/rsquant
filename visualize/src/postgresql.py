import psycopg2


class PgOption:
    def __init__(
        self, db_name: str, db_user: str, db_passwd: str, db_host: str, db_port: str
    ) -> None:
        self.db_name = db_name
        self.db_user = db_user
        self.db_passwd = db_passwd
        self.db_host = db_host
        self.db_port = db_port


class PgConnection:
    def __init__(self, option: PgOption) -> None:
        self.conn = psycopg2.connect(
            database=option.db_name,
            user=option.db_user,
            password=option.db_passwd,
            host=option.db_host,
            port=option.db_port,
        )
        self.cursor = self.conn.cursor()

    def __del__(self):
        self.cursor.close()
        self.conn.close()

    def execute(self, sql: str) -> list[tuple[any, ...]]:
        self.cursor.execute(sql)
        return self.cursor.fetchall()


def setup_postgres() -> PgConnection:
    pg_option = PgOption(
        db_name="quant_trader",
        db_user="postgres",
        db_passwd="postgres",
        db_host="localhost",
        db_port="5432",
    )
    return PgConnection(pg_option)
