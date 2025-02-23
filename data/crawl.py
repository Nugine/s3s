from pathlib import Path
from pprint import pprint  # noqa: F401
import re
import json

from bs4 import BeautifulSoup
import requests
import typer

cli = typer.Typer()

model_dir = Path(__file__).parent


def save_json(path, data):
    with open(path, "w") as f:
        json.dump(data, f, indent=4)


def download_aws_sdk(service: str, *, commit: str):
    url = f"https://github.com/awslabs/aws-sdk-rust/raw/{commit}/aws-models/{service}.json"
    resp = requests.get(url)
    assert resp.status_code == 200
    assert resp.json()
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
def crawl_error_codes():
    url = "https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html"

    html = requests.get(url).text

    soup = BeautifulSoup(html, "lxml")

    h2_id = "SelectObjectContentErrorCodeList"

    h2 = soup.css.select(f"#{h2_id}")[0]  # type:ignore

    # find the next table
    table = None
    for e in h2.next_elements:
        if e.name == "table":  # type:ignore
            table = e
            break
    assert table is not None

    th_list = table.css.select("th")  # type:ignore
    assert th_list[0].text == "Error code"
    assert th_list[1].text == "Description"
    assert th_list[2].text == "HTTP status code"

    tr_list = table.css.select("tr")[1:]  # type:ignore
    tr_list = [[e for e in tr.children if e.name == "td"] for tr in tr_list]

    ans = []
    for td_list in tr_list:
        t0 = td_list[0].css.select("code")[0].text
        t1 = td_list[1].text
        t2 = td_list[2].text

        error_code = t0

        description = re.sub(r"\n\t+", " ", t1).strip()

        if t2 == "405 Method Not Allowed":
            http_status_code = 405
        else:
            http_status_code = int(t2)

        ans.append(
            {
                "error_code": error_code,
                "description": description,
                "http_status_code": http_status_code,
            }
        )

    save_json(model_dir / "s3_error_codes.json", {"SelectObjectContent": ans})


@cli.command()
def update():
    download_s3_model()
    download_sts_model()
    crawl_error_codes()


if __name__ == "__main__":
    cli()
