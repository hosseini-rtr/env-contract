from pathlib import Path
import yaml


def find_schema(filename: str = "config.schema.yaml", root: Path | None = None) -> Path:
    root = root or Path.cwd()
    path = root / filename

    if not path.is_file:
        raise FileNotFoundError(f"Configuration Schema not found:")
    return path


def sync_schema(root: Path | None = None): ...


project_root = Path.cwd()
schema_file = project_root / "config.schema.yaml"

if not schema_file.exists():
    raise FileExistsError(f"Could not find {schema_file}")

with schema_file.open() as f:
    schema = yaml.safe_load(f)


def main(): ...
