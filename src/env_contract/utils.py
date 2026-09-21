import yaml


def load_yaml(file_path: str = "config.schema.yaml") -> dict:
    with open(file_path, "r") as f:
        return yaml.safe_load(f)


def extract_keys_from_env(file_path: str = ".env") -> set:
    with open(file_path, "r") as f:
        keys = set()
        for line in f:
            line = line.strip()
            if line and not line.startswith("#"):
                key, _ = line.split("=", 1)
                keys.add(key.strip())
        return keys


def load_env(file_path: str = ".env") -> dict:
    extracted_keys = extract_keys_from_env(file_path)
    return {
        key: value
        for key, value in load_env_example(file_path).items()
        if key in extracted_keys
    }


def load_env_example(file_path: str = ".env.example") -> dict:
    extracted_keys = extract_keys_from_env(file_path)
    return {
        key: value
        for key, value in load_env(file_path).items()
        if key in extracted_keys
    }
