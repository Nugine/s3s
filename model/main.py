from pathlib import Path
import requests
import typer

cli = typer.Typer()

model_dir = Path(__file__).parent


def download_aws_sdk(service: str, *, commit: str):
    url = f"https://github.com/awslabs/aws-sdk-rust/raw/{commit}/aws-models/{service}.json"
    resp = requests.get(url)
    with open(model_dir / f"{service}.json", "w") as f:
        f.write(resp.text)


@cli.command()
def download_s3_model():
    # https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/s3.json
    download_aws_sdk("s3", commit="c2fc7c405d0eb54590c9661cc78ef7c90bb951bc")


@cli.command()
def download_sts_model():
    # https://github.com/awslabs/aws-sdk-rust/commits/main/aws-models/sts.json
    download_aws_sdk("sts", commit="dd960806f609eb4508faa690b16db6f5f03a612c")


@cli.command()
def update():
    download_s3_model()
    download_sts_model()


if __name__ == "__main__":
    cli()
