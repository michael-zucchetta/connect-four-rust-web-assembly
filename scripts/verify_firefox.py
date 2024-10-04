#!/usr/bin/env python3
import time

from selenium import webdriver
from selenium.webdriver import ActionChains
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.support.ui import WebDriverWait


def dispatch_canvas_click(driver, canvas, column):
    ActionChains(driver).move_to_element_with_offset(canvas, 18 + column * 50, 35).click().perform()


def yellow_pixel_count(driver):
    return driver.execute_script(
        """
        const canvas = document.getElementById("connect-four-canvas");
        const data = canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height).data;
        let count = 0;
        for (let index = 0; index < data.length; index += 4) {
            if (data[index] > 235 && data[index + 1] > 190 && data[index + 1] < 230 && data[index + 2] < 130) {
                count += 1;
            }
        }
        return count;
        """
    )


def main():
    options = Options()
    options.add_argument("-headless")
    options.set_preference("browser.cache.disk.enable", False)
    options.set_preference("browser.cache.memory.enable", False)
    options.set_preference("browser.cache.offline.enable", False)
    options.set_preference("network.http.use-cache", False)

    driver = webdriver.Firefox(options=options)
    try:
        driver.set_window_size(1100, 800)
        driver.get(f"http://localhost:8000/?webdriver={time.time()}")

        WebDriverWait(driver, 15).until(
            lambda d: d.execute_script('return typeof window.initializeGame === "function"')
        )
        driver.execute_script("document.getElementsByName('difficulty')[0].value = 'VeryEasy';")
        driver.find_element(By.ID, "submit").click()

        canvas = WebDriverWait(driver, 15).until(
            EC.presence_of_element_located((By.ID, "connect-four-canvas"))
        )
        WebDriverWait(driver, 15).until(
            lambda d: d.execute_script(
                'return document.getElementById("canvas-status") !== null'
            )
        )
        starter = WebDriverWait(driver, 15).until(
            lambda d: d.execute_script(
                'return document.getElementById("coin-result")?.textContent || "";'
            )
        )
        if starter not in ("Player starts", "AI starts"):
            raise SystemExit(f"Unexpected coin result label: {starter!r}")
        yellow_pixels_before_spin = yellow_pixel_count(driver)
        time.sleep(1.0)
        coin_color = driver.execute_script(
            'return getComputedStyle(document.querySelector("#coin-toss .coin")).backgroundColor;'
        )
        expected_coin_color = "rgb(255, 95, 86)" if starter == "Player starts" else "rgb(255, 216, 102)"
        if coin_color != expected_coin_color:
            raise SystemExit(
                f"Unexpected coin color for {starter!r}: {coin_color!r}, expected {expected_coin_color!r}"
            )
        driver.save_screenshot("/tmp/connect-four-firefox-start.png")
        yellow_pixels_before_click = yellow_pixel_count(driver)
        if starter == "AI starts" and yellow_pixels_before_spin != 0:
            raise SystemExit("AI starts but yellow opening fiche appeared before the coin finished")
        if starter == "AI starts" and yellow_pixels_before_click == 0:
            raise SystemExit("AI starts but no yellow opening fiche was detected")

        winner = ""
        columns = [0, 1, 2, 3, 4, 5, 6, 0, 2, 4, 6, 1, 3, 5]
        for move in range(120):
            dispatch_canvas_click(driver, canvas, columns[move % len(columns)])
            time.sleep(0.08)
            winner = driver.execute_script(
                """
                return document.getElementById("canvas-status")?.textContent
                    || document.getElementById("game-status")?.textContent
                    || "";
                """
            )
            if "wins" in winner:
                break

        fireworks_count = driver.execute_script(
            'return document.querySelectorAll(".fireworks-layer .firework-burst span").length;'
        )
        driver.save_screenshot("/tmp/connect-four-firefox-verify.png")
        print(f"winner={winner!r}")
        print(f"starter={starter!r}")
        print(f"coin_color={coin_color!r}")
        print(f"yellow_pixels_before_spin={yellow_pixels_before_spin}")
        print(f"yellow_pixels_before_click={yellow_pixels_before_click}")
        print(f"fireworks_sparks={fireworks_count}")
        print("start_screenshot=/tmp/connect-four-firefox-start.png")
        print("screenshot=/tmp/connect-four-firefox-verify.png")

        if "wins" not in winner:
            raise SystemExit("No winner text detected after automated clicks")
        if fireworks_count < 36:
            raise SystemExit("Fireworks overlay did not render expected sparks")
    finally:
        driver.quit()


if __name__ == "__main__":
    main()
