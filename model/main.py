from pathlib import Path
import requests
import typer

cli = typer.Typer()

model_dir = Path(__file__).parent


@cli.command()
def download_s3_model():
    # https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json

    commit = "c2fc7c405d0eb54590c9661cc78ef7c90bb951bc"
    url = f"https://github.com/awslabs/aws-sdk-rust/raw/{commit}/aws-models/s3.json"

    resp = requests.get(url)
    with open(model_dir / "s3.json", "w") as f:
        f.write(resp.text)


@cli.command()
def update():
    download_s3_model()


if __name__ == "__main__":
    cli()
